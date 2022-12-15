use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let ylevel = if args.example { 10 } else { 2_000_000 };

    let sensors = args.input.lines().map(|l| {
            let mut nums = l.split(&['=', ',', ':'])
            .filter_map(|c| c.parse::<i32>().ok());
            let sensor = [nums.next().unwrap(),nums.next().unwrap()];
            let beacon = [nums.next().unwrap(),nums.next().unwrap()];
            let md = manhattan_distance(&sensor, &beacon);
            (sensor, md)
    });

    let solution = if !args.second {
        let (from, to) = sensors.fold((i32::MAX, i32::MIN), |(from, to), (s,md)| -> (i32, i32) {
            let (my,py) = (s[1] - md, s[1] + md);
            if (my..=py).contains(&ylevel) {
                let mx = s[0] - (md - (ylevel - s[1]).abs());
                let px = s[0] + (md - (ylevel - s[1]).abs());
                (from.min(mx), to.max(px))
            } else {
                (from, to)
            }
        });
        from.abs() + to
    } else {
        todo!("part2")
    };

    result(solution, now.elapsed(), &args)
}

#[inline]
fn manhattan_distance(sensor: &[i32; 2], beacon: &[i32; 2]) -> i32 {
    let dx = (beacon[0] - sensor[0]).abs();
    let dy = (beacon[1] - sensor[1]).abs();
    dx + dy
}
