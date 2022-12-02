use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;

    let scores = args
        .input
        .iter()
        .map(|round| {
            // … => [part1, part2]
            // item + outcome
            // item: rock=1, paper=2, scissor=3
            // outcome: lose=0, draw=3, win=6
            match (round.chars().nth(0), round.chars().nth_back(0)) {
                (Some('A'), Some('X')) => [1 + 3, 3 + 0],
                (Some('A'), Some('Y')) => [2 + 6, 1 + 3],
                (Some('A'), Some('Z')) => [3 + 0, 2 + 6],
                (Some('B'), Some('X')) => [1 + 0, 1 + 0],
                (Some('B'), Some('Y')) => [2 + 3, 2 + 3],
                (Some('B'), Some('Z')) => [3 + 6, 3 + 6],
                (Some('C'), Some('X')) => [1 + 6, 2 + 0],
                (Some('C'), Some('Y')) => [2 + 0, 3 + 3],
                (Some('C'), Some('Z')) => [3 + 3, 1 + 6],
                (_, _) => [0, 0],
            }
        })
        .fold([0, 0], |[score1, score2], [part1, part2]| {
            [score1 + part1, score2 + part2]
        });

    let solution = scores[part(&args)];

    if args.example {
        example_output(&args, solution);
    } else {
        println!("solution: {solution}");
    }
    Ok(())
}
