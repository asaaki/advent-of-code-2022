use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

type N = u32;

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let elves = args.input.split("\n\n").map(|bag| {
        bag.split('\n')
            .filter_map(|c| c.parse::<N>().ok())
            .sum::<N>()
    });

    let solution: N = if !args.second {
        elves.max().unwrap().to_owned()
    } else {
        // easiest way to get the top3
        let mut elves: Vec<_> = elves.collect();
        elves.sort_unstable_by(|a, b| b.cmp(a));
        elves.iter().take(3).sum()
    };

    result(solution, now.elapsed(), &args)
}
