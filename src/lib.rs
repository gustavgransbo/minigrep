use std::fs;
use std::error::Error;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;
    println!("File content: {}", file_content);
    Ok(())
}
