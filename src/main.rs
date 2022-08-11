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

    for line in content.lines() {
        let l = line.expect("Unable to read line");
        if l.contains(&args.pattern) {
            println!("{}", l)
        }
    }
}
