use crate::util::PAGE_SIZE;

#[derive(Debug)]
pub enum PageType {
    Free,
    Table,
    Index,
}

#[derive(Debug)]
pub struct Page {
    pub id: u32,
    pub data: [u8; PAGE_SIZE],
}
