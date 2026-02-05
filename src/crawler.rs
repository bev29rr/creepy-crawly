use reqwest::{blocking::get};
use std::error::Error;

const DEFAULT_DEPTH: &str = "5";

pub fn from(opt_url: Option<&String>, opt_depth: Option<&String>) -> Result<(), Box<dyn Error>> {
    let default_depth_str: &String = &DEFAULT_DEPTH.to_string();
    let depth_str = opt_depth.unwrap_or(default_depth_str);
    if let Some(url) = opt_url {
        let depth: u32 = depth_str.parse()?;

        let raw_page = scrape(url)?;

        println!("{}, {}, {}", url, depth, raw_page);
        Ok(())
    } else {
        Err(Box::from("No url provided"))
    }
}

fn scrape(url: &String) -> Result<String, Box<dyn Error>> {
    let response = get(url);
    let body = response?.text()?;
    Ok(body)
}

fn filter_urls(raw_page: String) {

}