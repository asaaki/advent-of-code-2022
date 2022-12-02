use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

type Num = u32;

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let mut elves: Vec<Num> = args
        .input
        .split("\n\n")
        .map(|bag| bag.split("\n").filter_map(|c| c.parse::<Num>().ok()).sum())
        .collect();
    elves.sort_unstable_by(|a, b| b.cmp(a));

    let mut it = elves.iter();

    let solution = if !args.second {
        it.next().unwrap().to_owned()
    } else {
        it.take(3).sum()
    };

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
