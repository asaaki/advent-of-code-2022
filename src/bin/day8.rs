use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let x = args.input.lines().next().unwrap().len();
    let y = args.input.lines().count();
    let edge_count = (x + y) as u32 * 2 - 4; // _ _ | | -4 for overlapping corners

    let mut trees = Vec::with_capacity(y);
    for line in args.input.lines() {
        // SAFETY: data is ASCII numeric
        // NOTE: let's not bother with moving them down to 0-9
        trees.push(line.as_bytes());
    }

    let solution = if !args.second {
        trees[..y - 1].iter().enumerate().skip(1).fold(
            edge_count,
            |vv, (ty, l)| {
                l[..x - 1]
                    .iter()
                    .enumerate()
                    .skip(1)
                    .fold(vv, |v, (tx, t)| {
                        // hint: left || right || top || bottom
                        if l[..tx].iter().all(|n| n < t)
                            || l[tx + 1..].iter().all(|n| n < t)
                            || trees[..ty].iter().map(|l| &l[tx]).all(|n| n < t)
                            || trees[ty + 1..]
                                .iter()
                                .map(|l| &l[tx])
                                .all(|n| n < t)
                        {
                            v + 1
                        } else {
                            v
                        }
                    })
            },
        )
    } else {
        trees[..y - 1]
            .iter()
            .enumerate()
            .skip(1)
            .fold(0, |ss, (ty, l)| {
                l[..x - 1]
                    .iter()
                    .enumerate()
                    .skip(1)
                    .fold(ss, |s, (tx, t)| {
                        let (mut s_top, mut s_bottom, mut s_left, mut s_right) =
                            (0, 0, 0, 0);

                        for n in trees[..ty].iter().map(|l| &l[tx]).rev() {
                            s_top += 1;
                            if n >= t {
                                break;
                            }
                        }
                        for n in trees[ty + 1..].iter().map(|l| &l[tx]) {
                            s_bottom += 1;
                            if n >= t {
                                break;
                            }
                        }
                        for n in l[..tx].iter().rev() {
                            s_left += 1;
                            if n >= t {
                                break;
                            }
                        }
                        for n in l[tx + 1..].iter() {
                            s_right += 1;
                            if n >= t {
                                break;
                            }
                        }

                        (s_top * s_bottom * s_left * s_right).max(s)
                    })
            })
    };

    result(solution, now.elapsed(), &args)
}
