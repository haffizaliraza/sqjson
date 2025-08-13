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
//! - Filter records using a custom predicate: `filter(|val| ...)`
//! - Pagination for queries: `query_page(field, value, limit, offset)`
//! - Export query results to a JSON file: `export_query(field, value, path)`
//! - Export full database: `export_to_file(path)`
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
//!
//!     // Flush to disk
//!     db.flush()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Get Full Record
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
//! # fn run() -> Result<(), DbError> {
//! let db = YourDb::open("jsondb.db")?;
//! let users_age_30 = db.query("age", 30)?;
//! println!("Users with age 30: {:?}", users_age_30);
//!
//! let users_city_ny = db.query("city", "NY")?;
//! println!("Users in NY: {:?}", users_city_ny);
//! # Ok(()) }
//! ```
//!
//! ### Filter Records
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # fn run() -> Result<(), DbError> {
//! let db = YourDb::open("jsondb.db")?;
//! let older_than_24 = db.filter(|doc| doc["age"].as_u64().unwrap_or(0) > 24)?;
//! for (key, user) in older_than_24 {
//!     println!("{} => {}", key, user);
//! }
//! # Ok(()) }
//! ```
//!
//! ### Paginated Query
//!
//! ```rust
//! # use sqjson::{YourDb, DbError};
//! # fn run() -> Result<(), DbErr
