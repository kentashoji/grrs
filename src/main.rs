#![allow(unused)]

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;
use anyhow::{Context, Result};
use grrs::find_matches;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.to_str().unwrap()))?;
    let content = BufReader::new(file);

    grrs::find_matches(content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

#[test]
fn test_find_matches() {
    let content = "lorem ipsum\ndolor sit amet".as_bytes();
    let mut result = Vec::new();
    find_matches(content, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
