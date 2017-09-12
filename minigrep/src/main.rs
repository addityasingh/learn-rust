extern crate minigrep;

use std::env;
use std::process;
use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Unable to access config, {:?}", err);
        process::exit(1);
    });

    run(config);
}
