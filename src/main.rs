use std::{env, error::Error};
use rusqlite::Connection;
mod db; 
mod crawl;

#[derive(Debug)]
struct TestStruct {
    id: i32,
    name: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database.db")?;
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(command) => {
            match command.as_str() {
                "create" => db::build_db(conn),
                "crawl" => crawl::from(args.get(2)),
                "index" => db::index(conn, args.get(2)),
                _ => help()
            }
        }
        None => help()
    }
}

fn help() -> Result<(), Box<dyn Error>> {
    println!("Unrecognised command.");
    Ok(())
}