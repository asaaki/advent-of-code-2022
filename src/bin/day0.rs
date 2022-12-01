use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;
    dbg!(&args);

    // pre-compute inputs if needed and/or shared by solutions

    let solution: &str = if !args.second {
        todo!("part1")
    } else {
        todo!("part2")
    };

    if args.example {
        example_output(&args, solution);
    } else {
        println!("solution: {solution}");
    }
    Ok(())
}
