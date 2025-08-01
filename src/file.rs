use crate::util::PAGE_SIZE;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub fn open_and_resize<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let min_size = (PAGE_SIZE * 100) as u64;
    let metadata = file.metadata()?;
    if metadata.len() < min_size {
        file.set_len(min_size)?;
    }

    Ok(file)
}
