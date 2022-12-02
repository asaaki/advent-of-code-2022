use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let scores = args.input.as_bytes().chunks(4).fold(
        [0, 0],
        |[score1, score2], round| {
            // … => [part1, part2]
            // item + outcome
            // item: rock=A=X=1, paper=B=Y=2, scissor=C=Z=3
            // outcome: lose=0, draw=3, win=6
            match (round[0], round[2]) {
                (b'A', b'X') => [1 + 3 + score1, 3 + 0 + score2],
                (b'A', b'Y') => [2 + 6 + score1, 1 + 3 + score2],
                (b'A', b'Z') => [3 + 0 + score1, 2 + 6 + score2],
                (b'B', b'X') => [1 + 0 + score1, 1 + 0 + score2],
                (b'B', b'Y') => [2 + 3 + score1, 2 + 3 + score2],
                (b'B', b'Z') => [3 + 6 + score1, 3 + 6 + score2],
                (b'C', b'X') => [1 + 6 + score1, 2 + 0 + score2],
                (b'C', b'Y') => [2 + 0 + score1, 3 + 3 + score2],
                (b'C', b'Z') => [3 + 3 + score1, 1 + 6 + score2],
                _ => [score1, score2],
            }
        },
    );

    eprintln!("time: {:?}", now.elapsed());
    result(&args, scores[part(&args)])
}
