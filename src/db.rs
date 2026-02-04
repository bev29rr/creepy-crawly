use std::error::Error;
use rusqlite::{Connection, Result};

struct Websites {
    url: String,
    content: String
}

pub fn build_db(conn: Connection) -> Result<(), Box<dyn Error>> {
    conn.execute("CREATE TABLE websites (
        url         TEXT PRINARY KET NOT NULL,
        contents    TEXT
    )", ())?;

    Ok(())
}

pub fn index(conn: Connection, index_str: Option<&String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn test() {
    println!("Testy")
}