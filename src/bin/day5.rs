use aoc_lib::*;
use tinyvec::*;

const BIN: &str = env!("CARGO_BIN_NAME");

const STACK_CAP: usize = 64;
const MAX_STACKS: usize = 9;

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let (stacks, instructions) = args.input.split_once("\n\n").unwrap();

    let mut workset = array_vec![array_vec![' '; STACK_CAP]; MAX_STACKS];

    stacks
        .lines()
        .rev()
        .skip(1)
        .enumerate()
        .for_each(|(li, l)| {
            l.char_indices()
                .skip(1)
                .step_by(4)
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| workset[i / 4][li] = c);
        });
    for w in workset.iter_mut() {
        w.retain(|c| c.is_alphabetic());
    }

    let mut tmp_stack = ArrayVec::<[_; STACK_CAP]>::new();

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

    let solution: compact_str::CompactString =
        workset.iter().filter_map(|w| w.last()).collect();

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
