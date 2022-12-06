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
        .fold(None, |pos, (i, s)| {
            let check = (1..=(wsize - 1))
                .map(|j| !s[j..].contains(&s[j - 1]))
                .all(|b| b);
            if pos.is_none() && check {
                Some(i + wsize)
            } else {
                pos
            }
        })
        .unwrap();

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
