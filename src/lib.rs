use std::{env, error::Error};
use std::{env::Args, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        let query = "Rust: it's ok";
        let contents = "\
C++: it's ok
Python: it's ok
C: it's ok
Rust: it's ok
";
        assert_eq!(vec!["Rust: it's ok"], search(query, contents));
    }

    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
C++: it's ok
Python: it's ok
C: it's ok
Rust: it's ok
rust: it's ok
";
        assert_eq!(vec!["Rust: it's ok"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Rust";
        let contents = "\
C++: it's ok
Python: it's ok
C: it's ok
Rust: it's ok
rust: it's ok
";
        assert_eq!(
            vec!["Rust: it's ok", "rust: it's ok"],
            search_case_insensitive(query, contents)
        );
    }
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get filename string"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
