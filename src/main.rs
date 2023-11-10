use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{arg, Parser};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Pattern to search for
    #[arg(short, long, required = false, default_value_t = String::from("book"))]
    pattern: String,
    /// File to search in
    #[arg(short, long, required = false, default_value_t = String::from("haystack.txt"))]
    input: String,
}

fn main() {
    let args = Args::parse();
    let needle = Regex::new(&args.pattern).unwrap();
    let input = args.input;

    let f = File::open(input).unwrap();
    let (idx, line) = BufReader::new(f)
        .lines()
        .map_while(|line| line.ok())
        .enumerate()
        .find(|(_idx, line)| needle.is_match(line))
        .unwrap();

    println!("{}: {}", idx + 1, line);
}
