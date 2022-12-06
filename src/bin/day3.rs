use aoc_lib::*;
use std::ops::ControlFlow;

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
                // .try_for_each + ControlFlow for early returns on match
                let cf = left.chars().try_for_each(|c| {
                    if right.contains(c) {
                        return ControlFlow::Break(c);
                    }
                    ControlFlow::Continue(())
                });
                // without nightly we have to unpack it like that
                match cf {
                    ControlFlow::Break(c) => c,
                    _ => panic!("invariant violated, no matches"),
                }
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
                // .try_for_each + ControlFlow for early returns on match
                let cf = e1.chars().try_for_each(|c| {
                    if e2.contains(c) && e3.contains(c) {
                        return ControlFlow::Break(c);
                    }
                    ControlFlow::Continue(())
                });
                // without nightly we have to unpack it like that
                match cf {
                    ControlFlow::Break(c) => c,
                    _ => panic!("invariant violated, no matches"),
                }
            })
            .map(char2int)
            .sum()
    };

    result(solution, now.elapsed(), &args)
}

#[inline]
fn char2int(c: char) -> u32 {
    match c {
        c @ 'A'..='Z' => c as u32 - 38, // A=65, -65 +27
        c @ 'a'..='z' => c as u32 - 96, // a=97, -97 +1
        _ => 0, // we could also panic!(), doesn't matter here though
    }
}
