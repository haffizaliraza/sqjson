//! Embedded file-based JSON database engine using memory-mapped files.
//!
//! # sqjson
//!
//! `sqjson` is a lightweight, embedded, file-based key-value database written in Rust.
//! It stores structured data as JSON values and uses memory-mapped I/O for efficient read/write access.
//!
//! Inspired by SQLite, but minimal and JSON-native, with an emphasis on simplicity and portability.
//!
//! ## Features
//!
//! - Embedded, single-file storage (`.db` file)
//! - Efficient memory-mapped I/O for fast reads/writes
//! - JSON value storage using `serde_json`
//! - String-keyed key-value API: `put`, `get`, `delete`, `flush`
//! - Field-level access: `get_field(key, field)`
//! - Secondary indexes for fast field-based queries: `query(field, value)`
//! - Export to readable `.json` backup file
//! - Show all stored records
//!
//! ## Examples
//!
//! ### Open or Create a Database
//!
//! ```rust
//! use sqjson::{YourDb, DbError};
//! use serde_json::json;
//!
//! fn main() -> Result<(), DbError> {
//!     let mut db = YourDb::open("jsondb.db")?;
//!
//!     // Insert JSON records
//!     db.put("user:1", &json!({ "name": "Alice", "age": 30, "city": "NY" }))?;
//!     db.put("user:2", &json!({ "name": "Bob", "age": 25, "city": "LA" }))?;
//!     db.put("user:3", &json!({ "name": "Charlie", "age": 30, "city": "NY" }))?;
//!     db.put("user:4", &json!({ "name": "Diana", "age": 22, "city": "LA" }))?;
//!
//!     // Flush to disk
//!     db.flush()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Read a Full Record
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # fn run() -> Result<(), DbError> {
//! let db = YourDb::open("jsondb.db")?;
//! if let Some(user) = db.get("user:2")? {
//!     println!("User 2: {}", user);
//! }
//! # Ok(()) }
//! ```
//!
//! ### Get a Specific Field
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # fn run() -> Result<(), DbError> {
//! let db = YourDb::open("jsondb.db")?;
//! if let Some(age) = db.get_field("user:1", "age")? {
//!     println!("User 1 age is: {}", age);
//! }
//! # Ok(()) }
//! ```
//!
//! ### Query by Field (Secondary Index)
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # use serde_json::json;
//! # fn run() -> Result<(), DbError> {
//! let db = YourDb::open("jsondb.db")?;
//!
//! let users_age_30 = db.query("age", 30)?;
//! println!("Users with age 30: {:?}", users_age_30);
//!
//! let users_city_ny = db.query("city", "NY")?;
//! println!("Users in NY: {:?}", users_city_ny);
//! # Ok(()) }
//! ```
//!
//! ### Delete a Record
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # fn run() -> Result<(), DbError> {
//! let mut db = YourDb::open("jsondb.db")?;
//! db.delete("user:3")?;
//! println!("Deleted user:3");
//! # Ok(()) }
//! ```
//!
//! ### Export to JSON File
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # fn run() -> Result<(), DbError> {
//! let db = YourDb::open("jsondb.db")?;
//! db.export_to_file("backup.json")?;
//! println!("Exported to backup.json");
//! # Ok(()) }
//! ```
//!
//! ### Show All Records
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # fn run() -> Result<(), DbError> {
//! let db = YourDb::open("jsondb.db")?;
//! db.show_all()?;
//! # Ok(()) }
//! ```
//!
//! ## Limitations
//!
//! - Max value size limited to a single memory page (e.g. 4096 bytes)
//! - No concurrency or transactional safety (yet)
//! - Index and secondary indexes are rebuilt in memory on startup
//!
//! ## Planned Features
//!
//! - Page reuse and compaction
//! - Write-ahead logging (WAL)
//! - Persisted secondary indexes
//! - Range queries and filters
//! - CLI tooling
//!
//! ## License
//!
//! MIT or Apache-2.0




mod db;
mod pager;
mod page;
mod file;
mod error;
mod util;

pub use db::YourDb;
pub use error::DbError;
