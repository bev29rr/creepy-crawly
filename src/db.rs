use std::error::Error;
use rusqlite::{Connection, Result, fallible_streaming_iterator::FallibleStreamingIterator};

/*
struct Websites {
    url: String,
    content: String
}
*/

pub fn build_db(conn: Connection) -> Result<(), Box<dyn Error>> {
    conn.execute("CREATE TABLE websites (
        url         TEXT PRINARY KET NOT NULL,
        contents    TEXT
    )", ())?;

    Ok(())
}

pub fn populate(conn: Connection) -> Result<(), Box<dyn Error>>  {
    conn.execute("
        INSERT INTO websites (url, contents) 
        VALUES (?1, ?2)
    ", &["test", "abced"])?;
    Ok(())
}

pub fn index_count(conn: Connection, opt_index_str: Option<&String>) -> Result<(), Box<dyn Error>> {
    if let Some(index_str) = opt_index_str {
        let wrapped_index_str = format!("%{index_str}%");
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM websites WHERE contents LIKE ?1")?;
        let mut rows = stmt.query([wrapped_index_str])?;
        let row0: i64 = rows.nth(0)?.unwrap().get(0)?;

        println!("{:?}", row0);

        Ok(())
    } else {
        Err(Box::from("No index string provided"))
    }
}

pub fn check_url_is_new() -> Result<bool, Box<dyn Error>>  {
    Ok(true)
}

pub fn add_url(conn: &Connection, url: &String, contents: &String) -> Result<(), Box<dyn Error>> {
    conn.execute("
        INSERT INTO websites (url, contents) 
        VALUES (?1, ?2)
    ", &[url, contents])?;
    Ok(())
}