use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let mut regv = 1;

    let solution: String = if !args.second {
        let mut strip = Vec::with_capacity(250);
        strip.push(regv);
        for op in args.input.lines() {
            match op {
                "noop" => strip.push(regv),
                _ if op.starts_with("addx ") => {
                    let v: &i32 = &op[5..].parse().unwrap();
                    strip.push(regv);
                    strip.push(regv);
                    regv += v;
                }
                _ => panic!("shall not happen"),
            }
        }
        strip
            .iter()
            .enumerate()
            .skip(20)
            .step_by(40)
            .take(6)
            .fold(0, |acc, (i, v)| acc + (i as i32 * v))
            .to_string()
    } else {
        let mut crt = [32; 240]; // 32=' ' (better readability than '.')
        let mut cycle = 0;
        for op in args.input.lines() {
            match op {
                "noop" => {
                    pixel_work(regv, &mut cycle, &mut crt);
                }
                _ if op.starts_with("addx ") => {
                    pixel_work(regv, &mut cycle, &mut crt);
                    pixel_work(regv, &mut cycle, &mut crt);
                    regv += &op[5..].parse().unwrap();
                }
                _ => panic!("shall not happen"),
            }
        }
        crt.chunks(40)
            .map(|line| std::str::from_utf8(line).unwrap())
            .fold(String::with_capacity(250), |mut s, l| {
                s.push('\n');
                s.push_str(l);
                s
            })
    };

    result(solution, now.elapsed(), &args)
}

fn pixel_work(ri: i32, cycle: &mut i32, crt: &mut [u8; 240]) {
    let covered = (ri - 1..=ri + 1).contains(&(*cycle % 40));
    if covered {
        crt[*cycle as usize] = 35; // '#'
    }
    *cycle += 1;
}
