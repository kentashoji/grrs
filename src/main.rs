#![allow(unused)]

use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).expect("could not read file");
    let content = BufReader::new(file);

    find_matches(content, &args.pattern);
}

fn find_matches<R: BufRead>(content: R, pattern: &str) {
    for line in content.lines() {
        let l = line.expect("Unable to read line");
        if l.contains(pattern) {
            println!("{}", l);
        }
    }
}

#[test]
fn test_find_matches() {
    let content = "test".as_bytes();
    find_matches(content, "awesome");
}
