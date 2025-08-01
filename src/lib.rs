//! Embedded file-based database engine using memory-mapped files.

mod db;
mod pager;
mod page;
mod file;
mod error;
mod util;

pub use db::YourDb;
pub use error::DbError;
