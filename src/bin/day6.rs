use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let stream = args.input.as_bytes().windows(4).enumerate().fold(
        None,
        |pos, (i, s)| {
            let f1 = !s[1..].contains(&s[0]); // a not in bcd
            let f2 = !s[2..].contains(&s[1]); // b not in cd
            let f3 = s[2] != s[3]; // c in d
                                   // dbg!(pos, i, s, f1, f2, f3);
            if pos.is_none() && f1 && f2 && f3 {
                Some(i + 4)
            } else {
                pos
            }
        },
    );
    // dbg!(stream);

    let solution = if !args.second {
        args.input
            .as_bytes()
            .windows(4)
            .enumerate()
            .fold(None, |pos, (i, s)| {
                let f1 = !s[1..].contains(&s[0]); // a not in bcd
                let f2 = !s[2..].contains(&s[1]); // b not in cd
                let f3 = s[2] != s[3]; // c in d
                if pos.is_none() && f1 && f2 && f3 {
                    Some(i + 4)
                } else {
                    pos
                }
            })
            .unwrap()
    } else {
        args.input
            .as_bytes()
            .windows(14)
            .enumerate()
            .fold(None, |pos, (i, s)| {
                let f1 = !s[1..].contains(&s[0]);
                let f2 = !s[2..].contains(&s[1]);
                let f3 = !s[3..].contains(&s[2]);
                let f4 = !s[4..].contains(&s[3]);
                let f5 = !s[5..].contains(&s[4]);
                let f6 = !s[6..].contains(&s[5]);
                let f7 = !s[7..].contains(&s[6]);
                let f8 = !s[8..].contains(&s[7]);
                let f9 = !s[9..].contains(&s[8]);
                let f10 = !s[10..].contains(&s[9]);
                let f11 = !s[11..].contains(&s[10]);
                let f12 = !s[12..].contains(&s[11]);
                let f13 = s[12] != s[13];
                if pos.is_none()
                    && f1
                    && f2
                    && f3
                    && f4
                    && f5
                    && f6
                    && f7
                    && f8
                    && f9
                    && f10
                    && f11
                    && f12
                    && f13
                {
                    Some(i + 14)
                } else {
                    pos
                }
            })
            .unwrap()
    };
    // let solution = stream.unwrap();

    eprintln!("time: {:?}", now.elapsed());
    result(&args, solution)
}
