//! # Minigrep
//!
//! Minigrep is a small, useless, partial implementation of the `grep` utility.
//!


use std::env;
use std::error::Error;
use std::fs;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for hit in results {
        println!("{}", hit);
    }
    Ok(())
}


/// Configuration structure for command line options.
///
/// # Examples
/// ```
/// use minigrep::Config;
///
/// let conf = Config {
///     query: String::from("foo"),
///     filename: String::from("bar"),
///     case_sensitive: true,
/// };
///
/// assert_eq!(conf.case_sensitive, true);
/// ```
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {  // TODO: Add 'static lifetime to Err
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string missing"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename missing"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}


fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            line
                .to_lowercase()
                .contains(&query.to_lowercase())
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
