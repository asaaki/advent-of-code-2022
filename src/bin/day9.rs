use ahash::{HashSet, HashSetExt};
use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let mut visited: HashSet<Pos> = HashSet::with_capacity(10_000);
    let mut head = Pos::default();
    let mut tails = [Pos::default(); 9];

    for instruction in args.input.lines() {
        let (direction, amount) = instruction.split_once(' ').unwrap();
        for _ in 0..amount.parse().unwrap() {
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => unreachable!(),
            }
            follow(&head, &mut tails[0]);

            if !args.second {
                visited.insert(tails[0]);
            } else {
                for i in 1..9 {
                    // Learning from day 11: this can be avoided by using RefCell;
                    // the left side does not need to be mutable, we just follow the
                    // rabbi… erm the borrow checker here.
                    let (left, right) = tails.split_at_mut(i);
                    follow(&left[i - 1], &mut right[0]);
                }
                visited.insert(tails[8]);
            }
        }
    }

    result(visited.len(), now.elapsed(), &args)
}

fn follow(lead: &Pos, t: &mut Pos) {
    let dx = lead.x - t.x;
    let dy = lead.y - t.y;
    if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
        t.x += dx.signum();
        t.y += dy.signum()
    }
}
