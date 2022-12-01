use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;

    let input = args.input.join("\n");
    let inputs: Vec<_> = input.split("\n\n").collect();

    let mut inputs: Vec<isize> = inputs
        .iter()
        .map(|bag| bag.split("\n").map(|c| c.parse::<isize>().unwrap()).sum())
        .collect();

    inputs.sort();
    inputs.reverse();

    let solution = if !args.second {
        inputs.get(0).unwrap().to_owned()
    } else {
        inputs.iter().take(3).sum()
    };

    if args.example {
        example_output(&args, solution);
    } else {
        println!("solution: {solution}");
    }
    Ok(())
}
