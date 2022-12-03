use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let c2i = |v: char| {
        let v = if v.is_lowercase() {
            v as u32 - 'a' as u32 + 1
        } else if v.is_uppercase() {
            v as u32 - 'A' as u32 + 27
        } else {
            0
        };
        v
    };

    let part1: Vec<_> = args
        .input
        .split('\n')
        .filter(|&l| !l.is_empty())
        .map(|s| {
            // dbg!(l, l.len()/2);
            let (left, right) = s.split_at(s.len() / 2);
            // todo not save
            let mut v = '0';
            for l in left.chars() {
                for r in right.chars() {
                    if l == r {
                        v = l.clone();
                        break;
                    }
                }
            }
            v
        })
        .map(c2i)
        .collect();

    let part2: Vec<_> = args.input.split('\n').collect();
    // dbg!(&part2);
    let part2: Vec<_> = part2
        .chunks(3)
        .map(|group| {
            // todo not save
            let mut v = '0';
            for e1 in group[0].chars() {
                for e2 in group[1].chars() {
                    for e3 in group[2].chars() {
                        if e1 == e2 && e2 == e3 {
                            v = e1.clone();
                            break;
                        }
                    }
                }
            }
            v
        })
        .map(c2i)
        .collect();
    dbg!(&part2);

    let solution: u32 = if !args.second {
        part1.iter().sum()
    } else {
        part2.iter().sum()
    };

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
