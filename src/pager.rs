use crate::{error::DbError, file::open_and_resize, util::PAGE_SIZE};
use memmap2::MmapMut;
use std::fs::File;

pub struct Pager {
    mmap: MmapMut,
    file: File,
}

impl Pager {
    pub fn new(path: &str) -> Result<Self, DbError> {
        let file = open_and_resize(path)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self { mmap, file })
    }

    pub fn get_page(&self, page_id: u32) -> Result<&[u8], DbError> {
        let offset = (page_id as usize) * PAGE_SIZE;
        let end = offset + PAGE_SIZE;
        self.mmap.get(offset..end).ok_or(DbError::PageOutOfBounds)
    }

    pub fn write_page(&mut self, page_id: u32, data: &[u8]) -> Result<(), DbError> {
        if data.len() > PAGE_SIZE {
            return Err(DbError::Other("Data too large for page".into()));
        }

        let offset = (page_id as usize) * PAGE_SIZE;
        let end = offset + data.len();

        if end > self.mmap.len() {
            return Err(DbError::PageOutOfBounds);
        }

        self.mmap[offset..end].copy_from_slice(data);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), DbError> {
        self.mmap.flush()?;
        Ok(())
    }
}
