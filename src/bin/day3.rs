use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let part1: Vec<_> = args
        .input
        .trim_end()
        .split('\n')
        .map(|s| {
            let (left, right) = s.split_at(s.len() / 2);
            // invariant: both sides always have one common char
            left.chars()
                .reduce(|acc, c| if right.contains(c) { c } else { acc })
                .unwrap()
        })
        .map(char2int)
        .collect();

    let part2: Vec<_> = args.input.trim_end().split('\n').collect();
    let part2: Vec<_> = part2
        .chunks_exact(3)
        .map(|group| {
            // invariant: all three members always have one common char
            group[0]
                .chars()
                .reduce(|acc, c| {
                    if group[1].contains(c) && group[2].contains(c) {
                        c
                    } else {
                        acc
                    }
                })
                .unwrap()
        })
        .map(char2int)
        .collect();

    let solution: u32 = if !args.second {
        part1.iter().sum()
    } else {
        part2.iter().sum()
    };

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}

fn char2int(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        0
    }
}
