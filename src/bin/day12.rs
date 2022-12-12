use aoc_lib::*;
use pathfinding::prelude::dijkstra;

const BIN: &str = env!("CARGO_BIN_NAME");

type Map = Vec<Vec<u8>>;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, u8);

impl Pos {
    fn successors(
        &self,
        map: &Map,
        max_x: &usize,
        max_y: &usize,
    ) -> Vec<(Pos, usize)> {
        let &Pos(x, y, current) = self;
        let mut possible_moves = Vec::with_capacity(4);
        let max_h = current + 1;
        if y > 0 {
            let top = map.get(y - 1).and_then(|ly| ly.get(x)).unwrap();
            if top <= &max_h {
                possible_moves.push((Pos(x, y - 1, *top), 1));
            }
        }
        if y < *max_y {
            let bottom = map.get(y + 1).and_then(|ly| ly.get(x)).unwrap();
            if bottom <= &max_h {
                possible_moves.push((Pos(x, y + 1, *bottom), 1));
            }
        }
        if x > 0 {
            let left = map.get(y).and_then(|ly| ly.get(x - 1)).unwrap();
            if left <= &max_h {
                possible_moves.push((Pos(x - 1, y, *left), 1));
            }
        }
        if x < *max_x {
            let right = map.get(y).and_then(|ly| ly.get(x + 1)).unwrap();
            if right <= &max_h {
                possible_moves.push((Pos(x + 1, y, *right), 1));
            }
        }
        possible_moves
    }

    fn rev_successors(
        &self,
        map: &Map,
        max_x: &usize,
        max_y: &usize,
    ) -> Vec<(Pos, usize)> {
        let &Pos(x, y, current) = self;
        let mut possible_moves = Vec::with_capacity(4);
        let max_h = (Z - current) + 1;
        if y > 0 {
            let top = map.get(y - 1).and_then(|ly| ly.get(x)).unwrap();
            if (Z - top) <= max_h {
                possible_moves.push((Pos(x, y - 1, *top), 1));
            }
        }
        if y < *max_y {
            let bottom = map.get(y + 1).and_then(|ly| ly.get(x)).unwrap();
            if (Z - bottom) <= max_h {
                possible_moves.push((Pos(x, y + 1, *bottom), 1));
            }
        }
        if x > 0 {
            let left = map.get(y).and_then(|ly| ly.get(x - 1)).unwrap();
            if (Z - left) <= max_h {
                possible_moves.push((Pos(x - 1, y, *left), 1));
            }
        }
        if x < *max_x {
            let right = map.get(y).and_then(|ly| ly.get(x + 1)).unwrap();
            if (Z - right) <= max_h {
                possible_moves.push((Pos(x + 1, y, *right), 1));
            }
        }
        possible_moves
    }
}

const A: u8 = 'a' as u8;
const Z: u8 = 'z' as u8;

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let max_y = args.input.lines().count() - 1;
    let max_x = args.input.lines().next().unwrap().len() - 1;

    let mut map: Vec<Vec<_>> = Vec::with_capacity(max_y);

    let mut start_p1 = Pos::default();
    let mut hike_starts = Vec::default();
    let mut goal = Pos::default();

    for (y, line) in args.input.lines().enumerate() {
        let mut xline = Vec::with_capacity(max_x);
        for (x, c) in line.char_indices() {
            match c {
                'S' => {
                    start_p1 = Pos(x, y, A);
                    xline.push(A);
                    hike_starts.push(Pos(x, y, A))
                }
                'E' => {
                    goal = Pos(x, y, Z);
                    xline.push(Z);
                }
                h => {
                    if h == 'a' {
                        hike_starts.push(Pos(x, y, A))
                    }
                    xline.push(h as u8)
                }
            }
        }
        map.push(xline);
    }

    let solution: usize = if !args.second {
        dijkstra(
            &start_p1,
            |p| p.successors(&map, &max_x, &max_y),
            |p| *p == goal,
        )
        .unwrap()
        .1
    } else {
        dijkstra(
            &goal,
            |p| p.rev_successors(&map, &max_x, &max_y),
            |p| p.2 == A,
        )
        .unwrap()
        .1
    };

    result(solution, now.elapsed(), &args)
}
