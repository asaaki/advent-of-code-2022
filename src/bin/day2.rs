use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;

    let scores = args.input.iter().fold([0, 0], |[score1, score2], round| {
        // … => [part1, part2]
        // item + outcome
        // item: rock=A=X=1, paper=B=Y=2, scissor=C=Z=3
        // outcome: lose=0, draw=3, win=6
        match &round[..] {
                "A X" => [1 + 3 + score1, 3 + 0 + score2],
                "A Y" => [2 + 6 + score1, 1 + 3 + score2],
                "A Z" => [3 + 0 + score1, 2 + 6 + score2],
                "B X" => [1 + 0 + score1, 1 + 0 + score2],
                "B Y" => [2 + 3 + score1, 2 + 3 + score2],
                "B Z" => [3 + 6 + score1, 3 + 6 + score2],
                "C X" => [1 + 6 + score1, 2 + 0 + score2],
                "C Y" => [2 + 0 + score1, 3 + 3 + score2],
                "C Z" => [3 + 3 + score1, 1 + 6 + score2],
                _ => [score1, score2],
            }
    });

    let solution = scores[part(&args)];

    result(&args, solution)
}
