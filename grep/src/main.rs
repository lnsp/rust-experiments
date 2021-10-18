use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Write};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Use regex query
    let query = &args[1].as_str();
    let matcher = Regex::new(query).expect("invalid pattern");
    let mut stdin = BufReader::new(match args.len() {
        2 => Box::new(io::stdin()) as Box<dyn Read>,
        3 => Box::new(File::open(&args[2]).expect("can't open file")) as Box<dyn Read>,
        _ => panic!("invalid number of args"),
    });

    // Start matching files
    let mut buffer = String::new();
    loop {
        let bytes_read = stdin.read_line(&mut buffer).unwrap();
        if bytes_read == 0 {
            return;
        }
        if matcher.is_match(buffer.trim()) {
            io::stdout()
                .write_all(buffer.as_bytes())
                .unwrap_or_else(|error| match error.kind() {
                    ErrorKind::BrokenPipe => (),
                    _ => panic!("unexpected error: {}", error),
                });
        }
        buffer.clear();
    }
}
