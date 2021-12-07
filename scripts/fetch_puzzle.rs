#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! reqwest = "0.11.7"
//! tokio = { version = "1", features = ["full"] }
//! anyhow = "1.0.51"
//! ```
use reqwest::Client;
use reqwest::header::COOKIE;
use std::env;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let year = env::var("PUZZLE_YEAR").expect("PUZZLE_YEAR not defined");
    let day = env::var("PUZZLE_DAY").expect("PUZZLE_DAY not defined").parse::<i32>()?;
    let session = env::var("SESSION_COOKIE").expect("SESSION_COOKIE not defined");

    let client = Client::new();
    
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = client.get(url)
        .header(COOKIE, format!("session={}", session))
        .send()
        .await?
        .text()
        .await?;
    
    let path = format!("inputs/{}/puzzle_{:02}.input", year, day);
    let mut output = File::create(path)?;
    
    write!(output, "{}", resp).unwrap();

    Ok(())
}