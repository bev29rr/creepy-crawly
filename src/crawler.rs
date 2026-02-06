use reqwest::{blocking::get};
use scraper::{Html, Selector};
use texting_robots::{Robot, get_robots_url};
use std::error::Error;
use rusqlite::Connection;
use yansi::Paint;

use crate::db;

const DEFAULT_DEPTH: &str = "5";

pub fn from(conn: Connection, url: Option<&String>, depth: Option<&String>) -> Result<(), Box<dyn Error>> {
    let default_depth_str: &String = &DEFAULT_DEPTH.to_string();
    let depth_str = depth.unwrap_or(default_depth_str);
    if let Some(url) = url {
        let max_depth: u32 = depth_str.parse()?;
        iterate(&conn, url, 0, max_depth)?;
        Ok(())
    } else {
        Err(Box::from("No url provided"))
    }
}

fn iterate(conn: &Connection, url: &String, current_depth: u32, max_depth: u32) -> Result<(), Box<dyn Error>> {
    if db::check_url_is_new(conn, url)? == false {
        println!(
            "{} {}\n\t{}: {}",
            "Skipping".green(),
            url.underline(),
            "Reason".green(),
            "Already exists in DB".red()
        );
        return Ok(());
    } else if url.starts_with("http") == false {
        println!(
            "{} {}\n\t{}: {}",
            "Skipping".green(),
            url.underline(),
            "Reason".green(),
            "Not a valid url".red()
        );
        return Ok(());
    } else if check_robots_allowed(url)? == false {
        println!(
            "{} {}\n\t{}: {}",
            "Skipping".green(),
            url.underline(),
            "Reason".green(),
            "Robots aren't allowed".red()
        );
        return Ok(());
    }
    println!(
        "{} {}",
        "Scraping".green(),
        url.underline()
    );

    match scrape(url) {
        Ok(raw_page) => {
            db::add_url(conn, url, &raw_page)?;
            let urls = get_urls(raw_page)?;
            if current_depth < max_depth {
                for url in urls {
                    iterate(conn, &url, current_depth+1, max_depth)?;
                }
            }

            Ok(())
        }, 
        Err(_) => {
            println!(
                "{} {}\n\t{}: {}",
                "Failed to parse".green(),
                url.underline(),
                "Reason".green(),
                "Cannot access url".red()
            );
            Ok(())
        }
    }
}

fn scrape(url: &String) -> Result<String, Box<dyn Error>> {
    let response = get(url);
    let body = response?.text()?;
    Ok(body)
}

fn get_urls(raw_page: String) -> Result<Vec<String>, Box<dyn Error>> {
    let document = Html::parse_document(&raw_page);
    let link_selector = Selector::parse("a")?; 

    let mut links: Vec<String> = Vec::new();

    for element in document.select(&link_selector) {
        if let Some(link) = element.value().attr("href") && link != "" {
            links.push(link.to_string());
        }
    }

    Ok(links)
}

fn check_robots_allowed(url: &String) -> Result<bool, Box<dyn Error>> {
    let robots_url = get_robots_url(url)?;
    match scrape(&robots_url) {
        Ok(robots_txt) => {
            let r = Robot::new("", robots_txt.as_bytes())?;
            let allowed = r.allowed("https://www.rust-lang.org/ocean");

            Ok(allowed)
        },
        Err(_) => Ok(true) // when robots doesn't exist, assume true
    }
}