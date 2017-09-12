
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
    let mut content = String::new();
    file_handle.read_to_string(&mut content)?;

    println!("With text:\n{}", content);

    Ok(())
}

// Add this for reference
fn read_file (filepath: String) -> String {
    let mut file_handle = File::open(filepath).unwrap();
    let mut content = String::new();

    let _ = file_handle.read_to_string(&mut content)
        .expect("Cannot read file content");

    content
}