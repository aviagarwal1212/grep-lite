use std::cmp::max;

use regex::Regex;

fn main() {
    let ctx_lines = 3;
    let needle = Regex::new("picture").unwrap();

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
        .take(2 * ctx_lines + 1)
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
