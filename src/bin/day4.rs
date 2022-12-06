use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let solution: u32 = args
        .input
        .lines()
        .map(|assignment| {
            let mut splits = assignment
                .splitn(4, &[',', '-'])
                .map(|s| s.parse::<u8>().unwrap());
            let start1 = splits.next().unwrap();
            let end1 = splits.next().unwrap();
            let start2 = splits.next().unwrap();
            let end2 = splits.next().unwrap();

            if args.second {
                start1 <= end2 && start2 <= end1
            } else {
                let left = start1 <= start2 && end1 >= end2;
                let right = start2 <= start1 && end2 >= end1;
                left || right
            }
        })
        .fold(0, |acc, b| if b { acc + 1 } else { acc });

    result(solution, now.elapsed(), &args)
}
