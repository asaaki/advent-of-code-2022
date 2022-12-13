use aoc_lib::*;
use pathfinding::prelude::bfs;

const BIN: &str = env!("CARGO_BIN_NAME");

const A: u8 = 'a' as u8;
const E: u8 = 'E' as u8;
const S: u8 = 'S' as u8;
const Z: u8 = 'z' as u8;

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

type Map = Vec<Vec<u8>>;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &Map, rev: bool) -> Vec<Pos> {
        let &Pos(x, y) = self;
        let max_h = if rev { Z - map[y][x] } else { map[y][x] } + 1;
        let mut possible_moves = Vec::with_capacity(4);

        for (nx, ny) in NEIGHBOURS {
            if x == 0 && nx == -1 {
                continue;
            }
            if y == 0 && ny == -1 {
                continue;
            }
            if x == map[0].len() - 1 && nx == 1 {
                continue;
            }
            if y == map.len() - 1 && ny == 1 {
                continue;
            }

            let sx = ((x as i32) + nx) as usize;
            let sy = ((y as i32) + ny) as usize;
            if max_h >= (if rev { Z - map[sy][sx] } else { map[sy][sx] }) {
                possible_moves.push(Pos(sx, sy));
            }
        }

        possible_moves
    }
}

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let max_y = args.input.lines().count() - 1;
    let max_x = args.input.lines().next().unwrap().len() - 1;
    // https://www.reddit.com/r/adventofcode/comments/zjnruc/comment/izwf0ay/?context=3
    // pathfinding has a Matrix, which we could use here instead of this vecvec
    let mut map: Vec<Vec<_>> = Vec::with_capacity(max_y);

    let (mut start_p1, mut goal) = (Pos::default(), Pos::default());

    for (y, line) in args.input.lines().enumerate() {
        let mut xline = Vec::with_capacity(max_x);
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                &S => {
                    start_p1 = Pos(x, y);
                    xline.push(A);
                }
                &E => {
                    goal = Pos(x, y);
                    xline.push(Z);
                }
                h => xline.push(*h),
            }
        }
        map.push(xline);
    }

    let solution: usize = if !args.second {
        bfs(&start_p1, |p| p.successors(&map, false), |p| *p == goal)
            .unwrap()
            .len()
            - 1
    } else {
        bfs(&goal, |p| p.successors(&map, true), |p| map[p.1][p.0] == A)
            .unwrap()
            .len()
            - 1
    };
    result(solution, now.elapsed(), &args)
}
