use byteorder::{ByteOrder, LittleEndian};

pub const PAGE_SIZE: usize = 4096;

pub fn align_to(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub fn u32_to_bytes_le(value: u32) -> [u8; 4] {
    let mut buf = [0u8; 4];
    LittleEndian::write_u32(&mut buf, value);
    buf
}

pub fn bytes_to_u32_le(bytes: &[u8]) -> u32 {
    LittleEndian::read_u32(bytes)
}
