use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let sa_pairs: [u32; 2] = args
        .input
        .lines()
        .map(|assignment| {
            // seems to enable some optimizations;
            // w/o: ~ 60 µs, with: ~ 50 µs, saved: 10 µs
            assert!(assignment.contains(&[',', '-']));

            let (elf1, elf2) = assignment.split_once(',').unwrap();
            let (start1, end1) = elf1.split_once('-').unwrap();
            let (start1, end1) = (
                start1.parse::<usize>().unwrap(),
                end1.parse::<usize>().unwrap(),
            );
            let (start2, end2) = elf2.split_once('-').unwrap();
            let (start2, end2) = (
                start2.parse::<usize>().unwrap(),
                end2.parse::<usize>().unwrap(),
            );

            let left = start1 <= start2 && end1 >= end2;
            let right = start2 <= start1 && end2 >= end1;
            let covered = left || right;

            let left_start_only = start1 >= start2 && start1 <= end2;
            let left_end_only = end1 >= start2 && end1 <= end2;
            let right_start_only = start2 >= start1 && start2 <= end1;
            let right_end_only = end2 >= start1 && end2 <= end1;
            let overlap = left_start_only
                || left_end_only
                || right_start_only
                || right_end_only;

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
