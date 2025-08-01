use std::collections::HashMap;
use serde_json::Value;
use crate::{pager::Pager, error::DbError};

pub struct YourDb {
    pager: Pager,
    index: HashMap<String, u32>, // maps key -> page_id
    next_page_id: u32,           // tracks next free page
}

impl YourDb {
    pub fn open(path: &str) -> Result<Self, DbError> {
        let mut pager = Pager::new(path)?;

        // Try to read index from page 0
        let index_data = pager.get_page(0)?;
        let index: HashMap<String, u32> = serde_json::from_slice(index_data).unwrap_or_default();

        // Find highest used page ID
        let next_page_id = index.values().copied().max().unwrap_or(0) + 1;

        Ok(Self {
            pager,
            index,
            next_page_id,
        })
    }

    pub fn put(&mut self, key: &str, value: &Value) -> Result<(), DbError> {
        let json_bytes = serde_json::to_vec(value)?;
        let len = json_bytes.len();

        if len + 4 > crate::util::PAGE_SIZE {
            return Err(DbError::Other("JSON too large for page".into()));
        }

        let mut page_data = vec![0u8; crate::util::PAGE_SIZE];
        page_data[..4].copy_from_slice(&(len as u32).to_le_bytes());
        page_data[4..4 + len].copy_from_slice(&json_bytes);

        let page_id = self.next_page_id;
        self.pager.write_page(page_id, &page_data)?;
        self.index.insert(key.to_string(), page_id);
        self.next_page_id += 1;
        Ok(())
    }


    pub fn get(&self, key: &str) -> Result<Option<Value>, DbError> {
        if let Some(&page_id) = self.index.get(key) {
            let data = self.pager.get_page(page_id)?;

            let len_bytes = &data[..4];
            let len = u32::from_le_bytes(len_bytes.try_into().unwrap()) as usize;

            let json = serde_json::from_slice(&data[4..4 + len]).ok(); // now safe
            Ok(json)
        } else {
            Ok(None)
        }
    }


    pub fn flush(&mut self) -> Result<(), DbError> {
        let index_bytes = serde_json::to_vec(&self.index)?;
        self.pager.write_page(0, &index_bytes)?;
        self.pager.flush()
    }

    
    pub fn show_all(&self) -> Result<(), DbError> {
        for (key, &page_id) in &self.index {
            let page = self.pager.get_page(page_id)?;
            let len = u32::from_le_bytes(page[..4].try_into().unwrap()) as usize;
            let json: serde_json::Value = serde_json::from_slice(&page[4..4 + len])?;
            println!("{} => {}", key, json);
        }
        Ok(())
    }



}
