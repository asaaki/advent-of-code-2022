use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let (stacks, instructions) = args.input.split_once("\n\n").unwrap();

    let stacks = stacks.lines();
    let max_slots = stacks
        .clone()
        .last()
        .unwrap()
        .matches(char::is_numeric)
        .count();

    let mut workset = vec![vec![' '; max_slots * 2]; max_slots];
    for (li, l) in stacks.rev().enumerate() {
        if l.chars().all(|c| !c.is_numeric()) {
            let x = &l[1..];
            for (ci, c) in x.char_indices() {
                if c.is_alphabetic() && ci % 4 == 0 {
                    workset[ci / 4][li - 1] = c;
                }
            }
        }
    }
    for w in workset.iter_mut() {
        w.retain(|c| c.is_alphabetic());
    }

    let instructions = instructions.lines();
    let moves: Vec<_> = instructions
        .map(|l| {
            let m = l.replace(|c: char| c.is_alphabetic(), " ");
            let mut m = m.trim().split_whitespace();
            let amount: usize = m.next().unwrap().parse().unwrap();
            let from: usize = m.next().unwrap().parse().unwrap();
            let to: usize = m.next().unwrap().parse().unwrap();
            [amount, from, to]
        })
        .collect();

    let mut tmp_c = Vec::new();
    for [i, from, to] in moves {
        if !args.second {
            for _ in 0..i {
                let c = workset[from - 1].pop().unwrap();
                workset[to - 1].push(c);
            }
        } else {
            let offset = workset[from - 1].len() - i;
            for c in workset[from - 1].drain(offset..) {
                tmp_c.push(c);
            }
            workset[to - 1].append(&mut tmp_c);
        }
    }
    let solution: String = workset.iter().filter_map(|w| w.last()).collect();

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
