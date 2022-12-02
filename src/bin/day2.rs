use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;

    let scores = args.input.iter().fold([0, 0], |[score1, score2], round| {
        // … => [part1, part2]
        // item + outcome
        // item: rock=A=X=1, paper=B=Y=2, scissor=C=Z=3
        // outcome: lose=0, draw=3, win=6
        match (round.chars().nth(0), round.chars().nth_back(0)) {
                (Some('A'), Some('X')) => [1 + 3 + score1, 3 + 0 + score2],
                (Some('A'), Some('Y')) => [2 + 6 + score1, 1 + 3 + score2],
                (Some('A'), Some('Z')) => [3 + 0 + score1, 2 + 6 + score2],
                (Some('B'), Some('X')) => [1 + 0 + score1, 1 + 0 + score2],
                (Some('B'), Some('Y')) => [2 + 3 + score1, 2 + 3 + score2],
                (Some('B'), Some('Z')) => [3 + 6 + score1, 3 + 6 + score2],
                (Some('C'), Some('X')) => [1 + 6 + score1, 2 + 0 + score2],
                (Some('C'), Some('Y')) => [2 + 0 + score1, 3 + 3 + score2],
                (Some('C'), Some('Z')) => [3 + 3 + score1, 1 + 6 + score2],
                (_, _) => [0, 0],
            }
    });

    let solution = scores[part(&args)];

    if args.example {
        example_output(&args, solution);
    } else {
        println!("solution: {solution}");
    }
    Ok(())
}
