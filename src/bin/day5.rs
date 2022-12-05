use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let (stacks, instructions) = args.input.split_once("\n\n").unwrap();

    let mut stacks = stacks.lines().peekable();
    let max_slots = stacks.peek().unwrap().len() / 4 + 1;

    let mut workset = vec![vec![' '; max_slots * 2]; max_slots];

    for (li, l) in stacks.rev().skip(1).enumerate() {
        let x = &l[1..];
        for (ci, c) in x.char_indices() {
            if c.is_alphabetic() && ci % 4 == 0 {
                workset[ci / 4][li] = c;
            }
        }
    }
    for w in workset.iter_mut() {
        w.retain(|c| c.is_alphabetic());
    }

    let mut tmp_stack = Vec::new();

    for l in instructions.lines() {
        let mut m = l
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());
        let amount = m.next().unwrap();
        let from = m.next().unwrap();
        let to = m.next().unwrap();

        for _ in 0..amount {
            let c = workset[from - 1].pop().unwrap();
            tmp_stack.push(c);
        }
        if args.second {
            tmp_stack.reverse();
        }
        workset[to - 1].append(&mut tmp_stack);
    }
    let solution: String = workset.iter().filter_map(|w| w.last()).collect();

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
