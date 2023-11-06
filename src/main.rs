use std::cmp::max;

use clap::{arg, Parser};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Pattern to search in haystack
    #[arg(short, long)]
    pattern: String,
    /// Number of rows to show for context in each direction
    #[arg(short, long, default_value_t = 2)]
    context: usize,
}

fn main() {
    let args = Args::parse();
    let ctx_lines: usize = args.context;
    let needle = Regex::new(&args.pattern).unwrap();

    let haystack = "\
Every face, every shop, 
bedroom window, public-house, and
dark square is a picture 
feverishly turned--in search of what?
It is the same with books.
What do we seek 
through millions of pages?";

    let queried_idx = haystack
        .lines()
        .position(|line| needle.is_match(line))
        .unwrap();

    let start_index = max(queried_idx as isize - ctx_lines as isize, 0) as usize;

    let output_chunk = haystack
        .lines()
        .skip(start_index)
        .take(queried_idx - start_index + ctx_lines + 1)
        .enumerate()
        .map(|(idx, item)| {
            let line_num = idx + start_index + 1;
            if line_num == queried_idx + 1 {
                return format!("* {} {}", line_num, item);
            }
            format!("  {} {}", line_num, item)
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", output_chunk);
}
