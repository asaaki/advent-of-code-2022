use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let sa_pairs: [u32; 2] = args
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

            let left = start1 <= start2 && end1 >= end2;
            let right = start2 <= start1 && end2 >= end1;
            let covered = left || right;

            let overlap = start1 <= end2 && start2 <= end1;

            (covered, overlap)
        })
        .fold([0, 0], |[acc_c, acc_o], (covered, overlap)| {
            let acc_c = if covered { acc_c + 1 } else { acc_c };
            let acc_o = if overlap { acc_o + 1 } else { acc_o };
            [acc_c, acc_o]
        });

    eprintln!("time: {:?}", now.elapsed());
    result(&args, sa_pairs[part(&args)])
}
