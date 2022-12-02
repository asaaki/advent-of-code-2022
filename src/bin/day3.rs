use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    dbg!(&args);

    // pre-compute inputs if needed and/or shared by solutions

    let solution: &str = if !args.second {
        todo!("part1")
    } else {
        todo!("part2")
    };

    result(&args, solution)
}
