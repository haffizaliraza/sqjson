use crate::util::PAGE_SIZE;

#[allow(dead_code)]
#[derive(Debug)]
pub enum PageType {
    Free,
    Table,
    Index,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Page {
    pub id: u32,
    pub data: [u8; PAGE_SIZE],
}
