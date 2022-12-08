use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let x = args.input.lines().next().unwrap().len();
    let y = args.input.lines().count();
    let edge_count = x as u32 * 2 + y as u32 * 2 - 4; // _ _ | | -4 for overlapping corners

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
                        let top =
                            trees[..ty].iter().map(|l| &l[tx]).all(|n| n < t);
                        let bottom = trees[ty + 1..]
                            .iter()
                            .map(|l| &l[tx])
                            .all(|n| n < t);
                        let left = l[..tx].iter().all(|n| n < t);
                        let right = l[tx + 1..].iter().all(|n| n < t);

                        if top || bottom || left || right {
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
                        let top = trees[..ty].iter().map(|l| &l[tx]);
                        let bottom = trees[ty + 1..].iter().map(|l| &l[tx]);
                        let left = l[..tx].iter();
                        let right = l[tx + 1..].iter();

                        let (mut s_top, mut s_bottom, mut s_left, mut s_right) = (0,0,0,0);

                        for n in top.rev() {
                            s_top += 1;
                            if n >= t {
                                break;
                            }
                        }
                        for n in bottom {
                            s_bottom += 1;
                            if n >= t {
                                break;
                            }
                        }
                        for n in left.rev() {
                            s_left += 1;
                            if n >= t {
                                break;
                            }
                        }
                        for n in right {
                            s_right += 1;
                            if n >= t {
                                break;
                            }
                        }
                        let score = s_top * s_bottom * s_left * s_right;

                        if score > s {
                            score
                        } else {
                            s
                        }
                    })
            })
    };

    result(solution, now.elapsed(), &args)
}
