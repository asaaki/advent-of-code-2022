use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let lines = args.input.lines();

    let solution: u32 = if !args.second {
        lines
            .clone()
            .map(|s| {
                let (left, right) = s.split_at(s.len() / 2);
                // invariant: both sides always have one common char
                left.chars()
                    .reduce(|acc, c| if right.contains(c) { c } else { acc })
                    .unwrap()
            })
            .map(char2int)
            .sum()
    } else {
        let part2e1 = lines.clone().step_by(3);
        let part2e2 = lines.clone().skip(1).step_by(3);
        let part2e3 = lines.skip(2).step_by(3);

        part2e1
            .zip(part2e2.zip(part2e3))
            .map(|(e1, (e2, e3))| {
                // invariant: all three members always have one common char
                e1.chars()
                    .reduce(|acc, c| {
                        if e2.contains(c) && e3.contains(c) {
                            c
                        } else {
                            acc
                        }
                    })
                    .unwrap()
            })
            .map(char2int)
            .sum()
    };

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}

#[inline]
fn char2int(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        0
    }
}
