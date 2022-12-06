use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let wsize = if args.second { 14 } else { 4 };
    let solution = args
        .input
        .as_bytes()
        .windows(wsize)
        .enumerate()
        // https://stackoverflow.com/a/46766782/653173
        .find(|(_, s)| !(1..s.len()).any(|i| s[i..].contains(&s[i - 1])))
        .unwrap()
        .0
        + wsize;

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
