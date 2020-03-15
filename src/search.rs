pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    };
    result
}


#[cfg(test)]
mod tests {

    use super::*;

    static TEST_CONTENT: &'static str = "\
Don't blame it on the sunshine
Don't blame it on the moonlight
Don't blame it on good times
Blame it on the boogie";
    
    #[test]
    fn one_match() {
        let query = "moon";
        assert_eq!(
            vec!["Don't blame it on the moonlight"],
            search(query, TEST_CONTENT)
        );
        
    }

    #[test]
    fn multiple_matches() {
        let query = "Don't";
        assert_eq!(
            vec![
                "Don't blame it on the sunshine",
                "Don't blame it on the moonlight",
                "Don't blame it on good times",
            ],
            search(query, TEST_CONTENT)
        );
    }

    #[test]
    fn no_match() {
        let query = "football";
        assert!(
            search(query, TEST_CONTENT).is_empty()
        );
    }
}
