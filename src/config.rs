use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments provided");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_too_few_arguments() {
        let args = [String::from("foo"), String::from("bar")];
        assert!(Config::new(&args).is_err());
    }

    #[test]
    fn new_config_ok() {
        let args = [String::from("foo"), String::from("bar"), String::from("baz")];
        let result = Config::new(&args);
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
        let args = [String::from("foo"), String::from("bar"), String::from("baz"), String::from("qux")];
        let result = Config::new(&args);
        match result {
            Ok(config) => {
                    assert_eq!(config.query, "bar");
                    assert_eq!(config.filename, "baz");
            }
            Err(err) => assert!(false, err)
        }
    }
}