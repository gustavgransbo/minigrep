pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|&x| x.contains(&query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content.lines()
        .filter(|&x| x.to_lowercase().contains(&query))
        .collect()
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
    fn case_sensitive_one_match() {
        let query = "moon";
        assert_eq!(
            vec!["Don't blame it on the moonlight"],
            search(query, TEST_CONTENT)
        );
    }

    #[test]
    fn case_sensitive_multiple_matches() {
        let query = "blame";
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
    fn case_sensitive_no_match() {
        let query = "football";
        assert!(
            search(query, TEST_CONTENT).is_empty()
        );
    }

    #[test]
    fn case_insensitive_one_match() {
        let query = "Moon";
        assert_eq!(
            vec!["Don't blame it on the moonlight"],
            search_case_insensitive(query, TEST_CONTENT)
        );
    }

    #[test]
    fn case_insensitive_multiple_matches() {
        let query = "blame";
        assert_eq!(
            vec![
                "Don't blame it on the sunshine",
                "Don't blame it on the moonlight",
                "Don't blame it on good times",
                "Blame it on the boogie",
            ],
            search_case_insensitive(query, TEST_CONTENT)
        );
    }

    #[test]
    fn case_insensitive_no_match() {
        let query = "football";
        assert!(
            search_case_insensitive(query, TEST_CONTENT).is_empty()
        );
    }
}
