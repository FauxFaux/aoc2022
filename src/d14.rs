use itertools::Itertools;
use num_integer::div_rem;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
struct Pos(usize);

const W: usize = 1024;

impl Pos {
    fn new((x, y): (i16, i16)) -> Self {
        Pos(y as usize * W + x as usize)
    }

    fn adj(self) -> [Pos; 3] {
        [Pos(self.0 + W), Pos(self.0 + W - 1), Pos(self.0 + W + 1)]
    }

    fn xy(self) -> (i16, i16) {
        let (y, x) = div_rem(self.0, W);
        (x as i16, y as i16)
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.xy())
    }
}

pub fn solve() {
    let lines = include_str!("d14.txt")
        .lines()
        .map(|x| x.split(" -> ").map(t).collect_vec())
        .collect_vec();

    let mut grid = bit_set::BitSet::with_capacity(900000);
    for line in &lines {
        let mut cursor = line[0];
        grid.insert(Pos::new(cursor).0);
        for coord in line {
            loop {
                match (cursor.0 - coord.0, cursor.1 - coord.1) {
                    (0, 0) => break,
                    (x, 0) => {
                        cursor.0 -= x.signum();
                    }
                    (0, y) => {
                        cursor.1 -= y.signum();
                    }
                    (_, _) => unreachable!("diagonal lines {cursor:?} {coord:?}"),
                }
                grid.insert(Pos::new(cursor).0);
            }
        }
    }

    let bottom = grid.iter().map(|p| Pos(p).xy().1).max().unwrap() as usize;
    let bottom_inner = Pos::new((0, (1 + bottom) as i16)).0;
    let mut rested = 0usize;

    let start = Pos::new((500, 0));

    while !grid.contains(start.0) {
        let mut sand: Pos = start;
        'sand: loop {
            if sand.0 > bottom_inner {
                grid.insert(sand.0);
                rested += 1;
                break 'sand;
            }
            for cand in sand.adj() {
                if !grid.contains(cand.0) {
                    // println!("{sand:?} -> {cand:?}");
                    sand = cand;
                    continue 'sand;
                }
            }
            grid.insert(sand.0);
            rested += 1;
            break;
        }
    }

    println!("{rested}");
}

fn t(s: &str) -> (i16, i16) {
    let (a, b) = s.split_once(',').unwrap();
    (i(a), i(b))
}

fn i(s: &str) -> i16 {
    s.parse().unwrap()
}
