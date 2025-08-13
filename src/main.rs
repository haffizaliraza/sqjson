use sqjson::{YourDb, DbError};
use serde_json::json;

fn main() -> Result<(), DbError> {
    // 🗂 Open or create the database
    let mut db = YourDb::open("jsondb.db")?;

    // ✅ Insert users
    db.put("user:1", &json!({ "name": "Alice", "age": 30, "city": "NY" }))?;
    db.put("user:2", &json!({ "name": "Bob", "age": 25, "city": "LA" }))?;
    db.put("user:3", &json!({ "name": "Charlie", "age": 30, "city": "NY" }))?;
    db.put("user:4", &json!({ "name": "Diana", "age": 22, "city": "LA" }))?;

    // 💾 Save to disk
    db.flush()?;

    // 📋 Show all records
    println!("\n-- All Records --");
    db.show_all()?;

    // 🔍 Get full object
    if let Some(user) = db.get("user:2")? {
        println!("\n✅ Found user:2: {}", user);
    }

    // 🎯 Get a specific field
    if let Some(age) = db.get_field("user:1", "age")? {
        println!("🧠 user:1 age is: {}", age);
    }

    // 🔍 Query by secondary index
    let users_age_30 = db.query("age", 30)?;
    println!("\n🔎 Users with age 30: {:?}", users_age_30);

    let users_city_ny = db.query("city", "NY")?;
    println!("🌆 Users in NY: {:?}", users_city_ny);

    let users_city_la = db.query("city", "LA")?;
    println!("🌆 Users in LA: {:?}", users_city_la);

    // 🔄 Use `filter()` to get users older than 24
    let older_than_24 = db.filter(|doc| doc["age"].as_u64().unwrap_or(0) > 24)?;
    println!("\n🎯 Users older than 24:");
    for (key, user) in older_than_24 {
        println!("{} => {}", key, user);
    }

    // 🔢 Use `query_page()` for pagination (e.g., first user in NY)
    let first_ny_user = db.query_page("city", "NY", 1, 0)?;
    println!("\n📄 First user in NY via pagination: {:?}", first_ny_user);

    // 📤 Use `export_query()` to export users in LA
    db.export_query("city", "LA", "la_users.json")?;
    println!("📦 Exported users in LA to la_users.json");

    // 🗑 Delete user
    db.delete("user:3")?;
    println!("\n🗑 Deleted user:3");

    // 💾 Save again
    db.flush()?;

    // 🧾 Export DB to backup file
    db.export_to_file("backup.json")?;
    println!("\n📤 Exported DB to backup.json");

    // 🔁 Final state
    println!("\n-- Final Records After Delete --");
    db.show_all()?;

    Ok(())
}
