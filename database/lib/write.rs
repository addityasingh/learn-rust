use std::fs::OpenOptions;
use std::io::{Write, Result};

pub fn append_to_log<P: AsRef<Path>>(content: &str) -> Result<()> {
    let input_file = "log.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    writeln!(file, "{}", content)
}
