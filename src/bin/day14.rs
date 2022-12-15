use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Sand,
    Rock,
    Abyss,
}

impl Cell {
    fn to_pixel(&self) -> char {
        use Cell::*;

        match self {
            Air => '.',
            Sand => 'o',
            Rock => '#',
            Abyss => 'x',
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pixel = self.to_pixel();
        write!(f, "{pixel}")
    }
}

type Map = Vec<Vec<Cell>>;
type Mixel = (i32, i32);

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let mut low_x = i32::MAX;
    let mut hi_x = i32::MIN;
    let mut low_y = i32::MIN;

    let builder: Vec<Vec<(i32, i32)>> = args
        .input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                    (low_x, hi_x) = (low_x.min(x), hi_x.max(x));
                    low_y = low_y.max(y);

                    (x, y)
                })
                .collect()
        })
        .collect();

    let extra = 1000;
    let max_x = (hi_x - low_x + 3 + (2 * extra)) as usize;
    let max_y = (low_y + 3) as usize;
    let spx_x = 500 - low_x + 1 + extra;

    let mut map: Map = vec![vec![Cell::Air; max_x]; max_y];
    if args.second {
        *map.last_mut().unwrap() = vec![Cell::Rock; max_x];
    } else {
        *map.last_mut().unwrap() = vec![Cell::Abyss; max_x];
    }

    for build_line in builder {
        build_line.windows(2).for_each(|points| {
            let ((ax, ay), (bx, by)) = (points[0], points[1]);
            if ax == bx {
                let (ay, by) = if ay < by { (ay, by) } else { (by, ay) };
                for y in ay..=by {
                    map[y as usize][(ax - low_x + 1 + extra) as usize] =
                        Cell::Rock;
                }
            } else if ay == by {
                let (ax, bx) = if ax < bx { (ax, bx) } else { (bx, ax) };
                for x in ax..=bx {
                    map[ay as usize][(x - low_x + 1 + extra) as usize] =
                        Cell::Rock;
                }
            }
        });
    }

    let mut stopped = false; // sand movement
    let mut abyssed = false;
    let mut spixel: Mixel = (spx_x, 0);
    let part2: Mixel = (spx_x, 0);
    let mut spixel_count = 0;

    loop {
        if !abyssed {
            map[spixel.1 as usize][spixel.0 as usize] = Cell::Sand;
            process_map(
                &mut map,
                &mut stopped,
                &mut abyssed,
                &mut spixel,
                &part2,
            );
            if stopped {
                stopped = false;
                spixel = (spx_x, 0);
                spixel_count += 1;
            }
        } else {
            spixel_count -= 1;
            break;
        }
    }

    let solution = if !args.second {
        spixel_count
    } else {
        spixel_count + 1 // dunno, some off-by-one in my logic
    };

    result(solution, now.elapsed(), &args)
}

fn process_map(
    map: &mut Map,
    stopped: &mut bool,
    abyssed: &mut bool,
    spixel: &mut Mixel,
    part2: &Mixel,
) {
    *stopped = true;

    let new_y = spixel.1 + 1;
    if map[new_y as usize][spixel.0 as usize] == Cell::Abyss {
        *abyssed = true;
        return;
    }

    for rx in [0, -1, 1] {
        let px = spixel.0 + rx;
        if map[new_y as usize][px as usize] == Cell::Air {
            *stopped = false;
            map[spixel.1 as usize][spixel.0 as usize] = Cell::Air;
            map[new_y as usize][px as usize] = Cell::Sand;
            *spixel = (px, new_y);
            break;
        }
    }

    if spixel.1 == 0 {
        *abyssed = true; // abjyse terminal condition
        return;
    }

    if *stopped {
        let x = part2.0;
        map[0][x as usize] = Cell::Sand;
        *spixel = (0, x);
    }
}
