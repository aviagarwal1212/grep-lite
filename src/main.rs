use std::{
    fs::File,
    io::{self, BufRead, BufReader},
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
    #[arg(short, long, required = false, default_value_t = String::from("-"))]
    input: String,
}

fn finder<T: BufRead + Sized>(reader: T, needle: Regex) {
    let (idx, line) = reader
        .lines()
        .map_while(|line| line.ok())
        .enumerate()
        .find(|(_idx, line)| needle.is_match(line))
        .unwrap();

    println!("{}: {}", idx + 1, line);
}

fn main() {
    let args = Args::parse();
    let needle = Regex::new(&args.pattern).unwrap();
    let input = args.input;

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        finder(reader, needle);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        finder(reader, needle);
    }
}
