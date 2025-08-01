use sqjson::{YourDb, DbError};
use serde_json::json;

fn main() -> Result<(), DbError> {
    let mut db = YourDb::open("jsondb.db")?;

    db.put("user:1", &json!({ "name": "Alice", "age": 30 }))?;
    db.put("user:2", &json!({ "name": "Bob", "age": 25 }))?;

    db.flush()?;

    println!("\n-- All Records --");
    db.show_all()?;

    if let Some(user) = db.get("user:1")? {
        println!("\nFound user 1: {}", user);
    } else {
        println!("\nUser 1 not found");
    }

    Ok(())
}
