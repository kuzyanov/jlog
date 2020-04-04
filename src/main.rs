use std::io;
use std::io::BufRead;
use jlog::cli;
use jlog::parser::{LogKeys, LogEntry};

fn main() {
    cli::parse_args();
    let log_keys = LogKeys::default();

    let reader = io::BufReader::new(io::stdin());

    for line in reader.lines() {
        let raw_line = line.expect("Error while read line");
        match LogEntry::parse(&log_keys, &raw_line) {
            Ok(log_entry) => println!("{}", log_entry.to_string()),
            Err(_) => println!("{}", raw_line)
        }
    }
}
