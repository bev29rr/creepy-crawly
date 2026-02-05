use std::{env, error::Error};
use rusqlite::Connection;
use yansi::Paint;
mod db; 
mod crawler;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database.db")?;
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(command) => {
            match command.as_str() {
                "create" => db::build_db(conn),
                "crawl" => crawler::from(conn, args.get(2), args.get(3)),
                "cindex" => db::index_count(conn, args.get(2)),
                _ => help()
            }
        }
        None => help()
    }
}

fn help() -> Result<(), Box<dyn Error>> {
    println!("Unrecognised command. Here is the list of recognised commands
    \tcargo run {} 
    \t└ {}
    \tcargo run {} {} {}
    \t└ {}
    \tcargo run {} {}
    \t└ {}",
    "create".bright_green(), "Creates the database".bright_blue(),
    "crawl".bright_green(), "[url]".bright_yellow(), "[depth]".bright_yellow(), "Begins crawling at a given url for a given depth, default is 5".bright_blue(),
    "cindex".bright_green(), "[index_string]".bright_yellow(), "Searches for occurences of the index_string in the database".bright_blue());
    Ok(())
}