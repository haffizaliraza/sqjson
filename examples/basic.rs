use sqjson::{YourDb, DbError};
use serde_json::json;

fn main() -> Result<(), DbError> {
    let mut db = YourDb::open("test.db")?;
    db.put("hello", &json!({ "msg": "world" }))?;
    db.flush()?;
    let val = db.get("hello")?;
    println!("{:?}", val);
    Ok(())
}
