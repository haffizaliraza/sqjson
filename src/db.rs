use std::collections::{HashMap, HashSet};
use serde_json::Value;
use crate::{pager::Pager, error::DbError};

pub struct YourDb {
    pager: Pager,
    index: HashMap<String, u32>, // maps key -> page_id
    secondary_indexes: HashMap<String, HashMap<Value, HashSet<String>>>, // field -> (value -> keys)
    next_page_id: u32,           // tracks next free page
}

impl YourDb {
    pub fn open(path: &str) -> Result<Self, DbError> {
        let pager = Pager::new(path)?;

        // Read index from page 0
        let index_data = pager.get_page(0)?;
        let index: HashMap<String, u32> = serde_json::from_slice(index_data).unwrap_or_default();

        let next_page_id = index.values().copied().max().unwrap_or(0) + 1;

        // Build secondary indexes by reading each record
        let mut secondary_indexes: HashMap<String, HashMap<Value, HashSet<String>>> = HashMap::new();
        for (key, &page_id) in &index {
            if let Ok(data) = pager.get_page(page_id) {
                if let Ok(len_bytes) = data[..4].try_into() {
                    let len = u32::from_le_bytes(len_bytes) as usize;
                    if let Ok(val) = serde_json::from_slice::<Value>(&data[4..4 + len]) {
                        if let Some(obj) = val.as_object() {
                            for (field, field_value) in obj {
                                let entry = secondary_indexes
                                    .entry(field.clone())
                                    .or_default()
                                    .entry(field_value.clone())
                                    .or_default();
                                entry.insert(key.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            pager,
            index,
            secondary_indexes,
            next_page_id,
        })
    }

    pub fn put(&mut self, key: &str, value: &Value) -> Result<(), DbError> {
        let json_bytes = serde_json::to_vec(value)?;
        if json_bytes.len() + 4 > crate::util::PAGE_SIZE {
            return Err(DbError::Other("JSON too large for page".into()));
        }

        // Remove old secondary index if key exists
        if let Some(existing_val) = self.get(key)? {
            if let Some(obj) = existing_val.as_object() {
                for (field, field_value) in obj {
                    if let Some(val_map) = self.secondary_indexes.get_mut(field) {
                        if let Some(keys) = val_map.get_mut(field_value) {
                            keys.remove(key);
                        }
                    }
                }
            }
        }

        let mut page_data = vec![0u8; crate::util::PAGE_SIZE];
        page_data[..4].copy_from_slice(&(json_bytes.len() as u32).to_le_bytes());
        page_data[4..4 + json_bytes.len()].copy_from_slice(&json_bytes);

        let page_id = self.next_page_id;
        self.pager.write_page(page_id, &page_data)?;
        self.index.insert(key.to_string(), page_id);
        self.next_page_id += 1;

        // Update secondary indexes
        if let Some(obj) = value.as_object() {
            for (field, field_value) in obj {
                let entry = self.secondary_indexes
                    .entry(field.clone())
                    .or_default()
                    .entry(field_value.clone())
                    .or_default();
                entry.insert(key.to_string());
            }
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Value>, DbError> {
        if let Some(&page_id) = self.index.get(key) {
            let data = self.pager.get_page(page_id)?;
            let len = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
            let json = serde_json::from_slice(&data[4..4 + len]).ok();
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

    pub fn delete(&mut self, key: &str) -> Result<(), DbError> {
        if let Some(existing_val) = self.get(key)? {
            if let Some(obj) = existing_val.as_object() {
                for (field, field_value) in obj {
                    if let Some(val_map) = self.secondary_indexes.get_mut(field) {
                        if let Some(keys) = val_map.get_mut(field_value) {
                            keys.remove(key);
                        }
                    }
                }
            }
        }

        if self.index.remove(key).is_some() {
            Ok(())
        } else {
            Err(DbError::Other("Key not found".into()))
        }
    }

    pub fn get_field(&self, key: &str, field: &str) -> Result<Option<Value>, DbError> {
        if let Some(val) = self.get(key)? {
            Ok(val.get(field).cloned())
        } else {
            Ok(None)
        }
    }

    pub fn filter<F>(&self, predicate: F) -> Result<Vec<(String, Value)>, DbError>
        where
            F: Fn(&Value) -> bool,
        {
            let mut results = Vec::new();
            for key in self.index.keys() {
                if let Some(val) = self.get(key)? {
                    if predicate(&val) {
                        results.push((key.clone(), val));
                    }
                }
            }
            Ok(results)
        }


    pub fn query_page(&self, field: &str, value: impl Into<Value>, limit: usize, offset: usize) -> Result<Vec<String>, DbError> {
        let keys = self.query(field, value)?;
        Ok(keys.into_iter().skip(offset).take(limit).collect())
    }


    pub fn export_query(&self, field: &str, value: impl Into<Value>, path: &str) -> Result<(), DbError> {
        let keys = self.query(field, value)?;
        let mut map = HashMap::new();
        for k in keys {
            if let Some(v) = self.get(&k)? {
                map.insert(k, v);
            }
        }
        std::fs::write(path, serde_json::to_string_pretty(&map)?)?;
        Ok(())
    }

    pub fn export_to_file(&self, path: &str) -> Result<(), DbError> {
        use std::fs;
        let mut map = HashMap::new();
        for (k, _) in &self.index {
            if let Some(v) = self.get(k)? {
                map.insert(k.clone(), v);
            }
        }
        let json = serde_json::to_string_pretty(&map)?;
        fs::write(path, json)?;
        Ok(())
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

    /// Query by field = value (from secondary index)
    pub fn query(&self, field: &str, value: impl Into<Value>) -> Result<Vec<String>, DbError> {
        let val = value.into();
        if let Some(val_map) = self.secondary_indexes.get(field) {
            if let Some(keys) = val_map.get(&val) {
                return Ok(keys.iter().cloned().collect());
            }
        }
        Ok(vec![])
    }
}
