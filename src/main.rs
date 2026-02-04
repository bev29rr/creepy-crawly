use std::error::Error;

use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct TestStruct {
    id: i32,
    name: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE test (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
        ()
    )?;

    let test1 = TestStruct {
        id: 0,
        name: "steve".to_string()
    };

    conn.execute("INSERT INTO test (name) VALUES (?1)", (&test1.name,))?;

    let mut stmt = conn.prepare("SELECT id, name FROM test")?;
    let test_str_iter = stmt.query_map([], |row| {
        Ok(TestStruct {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    test_str_iter.for_each(|struc| {
        println!("{:?}", struc);
    });
    Ok(())
}