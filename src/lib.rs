use std::fs;
use std::error::Error;

pub mod config;
pub mod search;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;

    for line in search::search(&config.query, &file_content) {
        println!("{}", line);
    }

    Ok(())
}
