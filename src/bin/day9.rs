use ahash::{HashSet, HashSetExt};
use aoc_lib::*;
use std::{cell::RefCell};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

const EMPTY: RefCell<Pos> = RefCell::new(Pos { x: 0, y: 0 });

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let mut visited: HashSet<Pos> = HashSet::with_capacity(10_000);
    let mut head = Pos::default();
    let tails = [EMPTY; 9];

    for instruction in args.input.lines() {
        let (direction, amount) = instruction.split_once(' ').unwrap();
        for _ in 0..amount.parse().unwrap() {
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("shall not happen"),
            }
            follow(&head, &mut tails[0].borrow_mut());

            if !args.second {
                visited.insert(tails[0].borrow().to_owned());
            } else {
                for i in 1..9 {
                    follow(&tails[i - 1].borrow(), &mut tails[i].borrow_mut());
                }
                visited.insert(tails[8].borrow().to_owned());
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
