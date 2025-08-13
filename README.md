[![Crates.io](https://img.shields.io/crates/v/sqjson.svg)](https://crates.io/crates/sqjson)
[![Documentation](https://docs.rs/sqjson/badge.svg)](https://docs.rs/sqjson)


# sqjson

**sqjson** is a simple, embedded, file-based key-value database using JSON values and memory-mapped files  
(like SQLite, but for JSON). Written in pure Rust with minimal dependencies (`serde`, `memmap2`, `thiserror`).

---

## 🚀 Features

- Lightweight and fast embedded JSON key-value store  
- File-based — stores data in a single `.db` file using fixed-size pages  
- Flexible JSON values with `serde_json`  
- Key-based storage and retrieval (`put`, `get`)  
- Secondary index support: query records by arbitrary JSON field values (`query`)  
- Filter records with custom predicates (`filter`)  
- Pagination for queries (`query_page`)  
- Export query results to JSON file (`export_query`)  
- Field-level access: fetch specific JSON fields without deserializing entire records (`get_field`)  
- Delete records by key (`delete`)  
- Export full database snapshot to pretty JSON file (`export_to_file`)  
- Memory-mapped I/O for efficient read/write performance  
- Simple API, easy to embed in Rust projects  
---

## 📦 Installation

Add to your Rust project with:

```sh
cargo add sqjson
```

Or add manually to your Cargo.toml:

```sh
[dependencies]
sqjson = "0.1"
```

## 🛠 Usage Examples
```sh
use sqjson::{YourDb, DbError};
use serde_json::json;

fn main() -> Result<(), DbError> {
    // Open or create the database
    let mut db = YourDb::open("jsondb.db")?;

    // Insert JSON records
    db.put("user:1", &json!({ "name": "Alice", "age": 30, "city": "NY" }))?;
    db.put("user:2", &json!({ "name": "Bob", "age": 25, "city": "LA" }))?;
    db.put("user:3", &json!({ "name": "Charlie", "age": 30, "city": "NY" }))?;
    db.put("user:4", &json!({ "name": "Diana", "age": 22, "city": "LA" }))?;

    // Persist changes
    db.flush()?;

    // Show all records
    println!("\n-- All Records --");
    db.show_all()?;

    // Get full record by key
    if let Some(user) = db.get("user:2")? {
        println!("\nFound user:2: {}", user);
    }

    // Get a specific JSON field
    if let Some(age) = db.get_field("user:1", "age")? {
        println!("user:1 age is: {}", age);
    }

    // Query by secondary index
    let users_age_30 = db.query("age", 30)?;
    println!("\nUsers with age 30: {:?}", users_age_30);

    // Filter records with a custom predicate
    let older_than_24 = db.filter(|val| val["age"].as_u64().unwrap_or(0) > 24)?;
    println!("\nUsers older than 24:");
    for (key, user) in older_than_24 {
        println!("{} => {}", key, user);
    }

    // Paginated query (first 1 user in NY)
    let first_ny_user = db.query_page("city", "NY", 1, 0)?;
    println!("\nFirst user in NY: {:?}", first_ny_user);

    // Export query results to JSON
    db.export_query("city", "LA", "la_users.json")?;
    println!("Exported LA users to la_users.json");

    // Delete a record
    db.delete("user:3")?;
    println!("\nDeleted user:3");

    // Persist delete operation
    db.flush()?;

    // Export full DB
    db.export_to_file("backup.json")?;
    println!("\nExported full database to backup.json");

    Ok(())
}
```

## 🔧 API Overview

Method	Description
YourDb::open(path)	Open or create a database file
put(key, value)	Insert or update a JSON value under a string key
get(key)	Retrieve a JSON value by key
get_field(key, field)	Retrieve a specific JSON field of a record
query(field, value)	Return list of keys where JSON field equals value
filter(predicate)	Return key-value pairs matching a custom predicate
query_page(field, value, limit, offset)	Paginated query results by field value
export_query(field, value, path)	Export query results to JSON file
delete(key)	Remove a record by key
flush()	Persist index and data pages to disk
show_all()	Print all stored key-value pairs (for debugging)
export_to_file(path)	Export entire DB contents to a pretty JSON file


## 📁 File Format
Page 0: Stores the index (mapping keys to page IDs)

Page 1 and onwards: Store actual JSON-encoded data

Fixed page size (default 4096 bytes)

Data stored as length-prefixed JSON blobs

## 📃 License
MIT OR Apache-2.0

## 👤 Author
Hafiz Ali Raza