use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args : impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("No query string provided")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename provided")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_too_few_arguments() {
        let args = ["foo", "bar"].iter().map(|s| s.to_string());
        assert!(Config::new(args).is_err());
    }

    #[test]
    fn new_config_ok() {
        let args = ["foo", "bar", "baz"].iter().map(|s| s.to_string());
        let result = Config::new(args);
        match result {
            Ok(config) => {
                    assert_eq!(config.query, "bar");
                    assert_eq!(config.filename, "baz");
            }
            Err(err) => assert!(false, err)
        }
    }

    #[test]
    fn new_config_extra_arguments() {
        let args = ["foo", "bar", "baz", "qux"].iter().map(|s| s.to_string());
        let result = Config::new(args);
        match result {
            Ok(config) => {
                    assert_eq!(config.query, "bar");
                    assert_eq!(config.filename, "baz");
            }
            Err(err) => assert!(false, err)
        }
    }
}