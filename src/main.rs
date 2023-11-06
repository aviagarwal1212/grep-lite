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

    let lines = haystack.lines().enumerate().collect::<Vec<(usize, &str)>>();
    let chunk = lines
        .windows(2 * ctx_lines + 1)
        .find(|&chunk| needle.is_match(chunk[ctx_lines].1))
        .unwrap_or_default();

    for (i, item) in chunk.iter().enumerate() {
        if i == ctx_lines {
            println!("* {} {}", item.0, item.1);
        } else {
            println!("  {} {}", item.0, item.1);
        }
    }
}
