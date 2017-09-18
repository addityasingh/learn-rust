extern crate ansi_term;

use ansi_term::Colour::Green;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Argument list is incomplete");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_insensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file_handle = File::open(config.filename)?;
    let mut contents = String::new();
    file_handle.read_to_string(&mut contents)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&contents, &config.query)
    } else {
        search(&contents, &config.query)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &'a str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut replacedLine: String;

    for line in contents.lines() {
        if line.contains(query) {
            let mut replacedVal = &Green.paint(query.clone()).to_string();
            replacedLine = str::replace(&line, &query, &replacedVal);
            result.push(replacedLine.trim().to_string());
        }
    }
    
    result
}

pub fn search_case_insensitive<'a>(
    contents: &'a str, 
    query: &'a str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut result: Vec<String> = vec![];
    let mut replacedLine: String;

    //TODO: Use search to acheive this
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            let mut replacedVal = &Green.paint(query.clone()).to_string();
            replacedLine = str::replace(&line, &query, &replacedVal);
            result.push(replacedLine.trim().to_string());
        }
    }
    
    result
}

mod tests {
    use super::*;

    #[test]
    fn search_single_result() {
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        let query = "duct";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(contents, query) 
        );
    }

    #[test]
    fn search_no_result() {
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        let query = "direct";
        
        assert_eq!(
            vec![] as Vec<&str>,
            search(contents, query) 
        );
    }

    #[test]
    fn search_case_insensitive_result() {
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me";
        let query = "rUsT";
        
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(contents, query) 
        );
    }
}