pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // to skip over program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path"),
        };

        Ok(Config { query, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_successfully() {
        let args: Vec<String> = ["minigrep", "query", "file"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        assert_eq!(None, Config::build(args.into_iter()).err());
    }

    #[test]
    fn no_query_build_fails() {
        let args: Vec<String> = ["minigrep"].iter().map(|&s| s.to_string()).collect();

        assert_eq!(
            Some("Did not get a query"),
            Config::build(args.into_iter()).err()
        );
    }

    #[test]
    fn no_file_build_fails() {
        let args: Vec<String> = ["minigrep", "query"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(
            Some("Did not get a file path"),
            Config::build(args.into_iter()).err()
        );
    }
}
