
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Argument list is incomplete");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // let file_content = read_file(config.filename);

    let mut file_handle = File::open(config.filename)?;
    let mut contents = String::new();
    file_handle.read_to_string(&mut contents)?;

    for line in search(&contents, &config.query) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            result.push(&line.trim());
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
}