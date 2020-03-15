use std::fs;
use std::error::Error;

pub mod config;
pub mod search;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search::search(&config.query, &file_content)
    } else {
        search::search_case_insensitive(&config.query, &file_content)
    };

    for line in  results{
        println!("{}", line);
    }

    Ok(())
}
