use std::{env, error::Error};
use rusqlite::Connection;
mod db; 
mod crawler;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database.db")?;
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(command) => {
            match command.as_str() {
                "create" => db::build_db(conn),
                "populate" => db::populate(conn),
                "crawl" => crawler::from(conn, args.get(2), args.get(3)),
                "cindex" => db::index_count(conn, args.get(2)),
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