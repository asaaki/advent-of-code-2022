use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;

    let input = args.input.join("\n");
    let inputs: Vec<_> = input.split("\n\n").collect();

    let mut inputs: Vec<u32> = inputs
        .iter()
        .map(|bag| bag.split("\n").map(|c| c.parse::<u32>().unwrap()).sum())
        .collect();

    inputs.sort();

    let solution = if !args.second {
        inputs.last().unwrap().to_owned()
    } else {
        inputs.iter().rev().take(3).sum()
    };

    if args.example {
        example_output(&args, solution);
    } else {
        println!("solution: {solution}");
    }
    Ok(())
}
