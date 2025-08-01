//! Embedded file-based database engine using memory-mapped files.

//! # sqjson
//!
//! `sqjson` is a simple, embedded, file-based key-value database written in Rust.
//! It stores data as JSON values and uses memory-mapped I/O for efficient access.
//! Inspired by SQLite but designed to be minimal and JSON-native.
//!
//! ## Features
//!
//! - Embedded, file-based storage (single `.db` file)
//! - Fast reads and writes using memory-mapped files
//! - JSON value storage powered by `serde_json`
//! - Simple API: open, put, get, flush
//! - Index-based key lookup (string keys only)
//!
//! ## Example
//!
//! ```rust
//! use sqjson::{YourDb, DbError};
//! use serde_json::json;
//!
//! fn main() -> Result<(), DbError> {
//!     let mut db = YourDb::open("my.db")?;
//!     db.put("user:1", &json!({ "name": "Alice", "age": 30 }))?;
//!     db.flush()?;
//!
//!     if let Some(user) = db.get("user:1")? {
//!         println!("User 1: {}", user);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Use Cases
//!
//! - Small desktop or CLI apps needing structured storage
//! - Learning resource for building file-based databases
//! - Lightweight data persistence without SQL
//!
//! ## Limitations
//!
//! - Max value size limited to 4096 bytes (1 page)
//! - No transactions or concurrency support (yet)
//! - Entire index is kept in memory
//!
//! ## License
//!
//! Licensed under MIT or Apache-2.0.


mod db;
mod pager;
mod page;
mod file;
mod error;
mod util;

pub use db::YourDb;
pub use error::DbError;
