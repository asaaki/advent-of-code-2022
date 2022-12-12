use std::cell::RefCell;

use aoc_lib::*;

type N = u64;

const BIN: &str = env!("CARGO_BIN_NAME");

enum Op {
    Add(N),
    Mul(N),
    Square,
}

struct Monkey {
    items: RefCell<Vec<N>>,
    op: Op,
    div: N,
    targets: [usize; 2],
}

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let monkeys = make_monkeys(&args);

    let solution = if args.second {
        let modulus: N = monkeys.iter().map(|m| m.div).product();
        solve(monkeys, 10_000, |x| x % modulus)
    } else {
        solve(monkeys, 20, |x| x / 3)
    };

    result(solution, now.elapsed(), &args)
}

fn make_monkeys(args: &Args) -> Vec<Monkey> {
    let mut monkeys = Vec::with_capacity(8);
    for def in args.input.split("\n\n") {
        let mut lines = def.lines().skip(1);
        let items: Vec<_> = lines.next().unwrap()[18..]
            .split(", ")
            .filter_map(|s| s.parse::<N>().ok())
            .collect();
        let (operator, num) =
            lines.next().unwrap()[23..].split_once(' ').unwrap();
        let div: N = lines
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let true_target: usize = lines
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let false_target: usize = lines
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let op = match operator {
            "+" => {
                if let Ok(v) = num.parse::<N>() {
                    Op::Add(v)
                } else {
                    Op::Mul(2)
                }
            }
            "*" => {
                if let Ok(v) = num.parse::<N>() {
                    Op::Mul(v)
                } else {
                    Op::Square
                }
            }
            _ => panic!("shall not happen"),
        };

        monkeys.push(Monkey {
            items: RefCell::new(items),
            div,
            op,
            targets: [true_target, false_target],
        });
    }
    monkeys
}

fn solve(monkeys: Vec<Monkey>, rounds: N, worry_fn: impl Fn(N) -> N) -> N {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for (i, m) in monkeys.iter().enumerate() {
            for item in m.items.borrow().iter() {
                inspections[i] += 1;
                let new_item = worry_fn(match m.op {
                    Op::Add(v) => item + v,
                    Op::Mul(v) => item * v,
                    Op::Square => item * item,
                });
                monkeys[m.targets[(new_item % m.div != 0) as usize]]
                    .items
                    .borrow_mut()
                    .push(new_item);
            }
            m.items.borrow_mut().clear();
        }
    }
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    let solution: N = inspections.iter().take(2).product();
    solution
}
