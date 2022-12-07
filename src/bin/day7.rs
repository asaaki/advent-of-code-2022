use aoc_lib::*;

type N = u32;
type HashMap = ahash::AHashMap<String, N>;

const BIN: &str = env!("CARGO_BIN_NAME");
const THRESHOLD: N = 100_000;
const DEVICE_MAX: N = 70_000_000;
const UPDATE: N = 30_000_000;

enum Mode {
    Cd,
    Ls,
}

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let terminal = args.input.lines();
    let mut mode = Mode::Cd;

    // NOTE: needed capacity measured from **my** inputs; avoids re-allocations during processing
    let mut cwd = String::with_capacity(70);
    let mut map = HashMap::with_capacity(167);

    for line in terminal {
        if line.starts_with('$') {
            if line.contains("cd") {
                mode = Mode::Cd;
                let dir = line.split_ascii_whitespace().skip(2).next().unwrap();
                match dir {
                    "/" => {
                        cwd = ".".to_owned() // use ".", so we can use "/" as delimiter instead
                    }
                    ".." => {
                        let new = cwd.rsplit_once('/').unwrap().0.to_owned();
                        cwd = new;
                    }
                    dirname => {
                        cwd.push('/');
                        cwd.push_str(dirname);
                    }
                }
            } else if line.contains("ls") {
                mode = Mode::Ls;
            } else {
                panic!("invalid command");
            }
        } else {
            match mode {
                Mode::Ls => {
                    if !line.starts_with("dir") {
                        let fsize: N = line
                            .split_ascii_whitespace()
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap();
                        let dsize = map.entry(cwd.clone()).or_insert(0);
                        *dsize += fsize;
                        cwd.match_indices('/').for_each(|(i, _)| {
                            let dsize =
                                map.entry(cwd[0..i].to_owned()).or_insert(0);
                            *dsize += fsize;
                        });
                    }
                }
                _ => panic!("shouldn't reach"),
            }
        }
    }

    let solution = if !args.second {
        map.values().filter(|&v| v <= &THRESHOLD).sum::<N>()
    } else {
        let needed = UPDATE - (DEVICE_MAX - map["."]);
        map.values()
            .filter(|&v| v >= &needed)
            .min()
            .unwrap()
            .to_owned()
    };

    result(solution, now.elapsed(), &args)
}
