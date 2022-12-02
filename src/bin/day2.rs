use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;

    // pre-compute inputs if needed and/or shared by solutions

    let rounds: Vec<_> = args.input.iter().map(|l| {
        let mut splits = l.splitn(2, ' ');
        [
            splits.next().unwrap().as_bytes()[0] as u32 - 64,
            splits.next().unwrap().as_bytes()[0] as u32 - 87
        ]
    }).collect();


    let solution: String = if !args.second {
        let scores: Vec<_> = rounds.iter().map(|[elf, me]| {
            // X 1 lose, Y 2 draw, Z 3 win
            match (elf, me) {
                (1 ,1) => 4,
                (1 ,2) => 8,
                (1 ,3) => 3,
                (2 ,1) => 1,
                (2 ,2) => 5,
                (2 ,3) => 9,
                (3 ,1) => 7,
                (3 ,2) => 2,
                (3 ,3) => 6,
                (_, _) => 0
            }
        }).collect();
        let part1: u32 = scores.iter().sum();
        part1.to_string()
    } else {
        let plays: Vec<_> = rounds.iter().map(|[elf, me]| {
            match (elf, me) {
                (1 ,1) => 3+0,
                (1 ,2) => 1+3,
                (1 ,3) => 2+6,
                (2 ,1) => 1+0,
                (2 ,2) => 2+3,
                (2 ,3) => 3+6,
                (3 ,1) => 2+0,
                (3 ,2) => 3+3,
                (3 ,3) => 1+6,
                (_, _) => 0
            }
        }).collect();
        let part2: u32 = plays.iter().sum();
        part2.to_string()
    };

    if args.example {
        example_output(&args, solution);
    } else {
        println!("solution: {solution}");
    }
    Ok(())
}
