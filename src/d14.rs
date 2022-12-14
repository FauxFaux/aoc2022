use itertools::Itertools;
use std::collections::HashSet;

type Pos = (i16, i16);

pub fn solve() {
    let lines = include_str!("d14.txt")
        .lines()
        .map(|x| x.split(" -> ").map(p).collect_vec())
        .collect_vec();

    let mut grid = HashSet::<Pos>::with_capacity(900_000);
    for line in &lines {
        let mut cursor = line[0];
        grid.insert(cursor);
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
                grid.insert(cursor);
            }
        }
    }

    let bottom = 1000 + *grid.iter().map(|(_, y)| y).max().unwrap();
    let mut rested = 0usize;

    while !grid.contains(&(500, 0)) {
        let mut sand: Pos = (500, 0);
        'sand: loop {
            if sand.1 == bottom {
                grid.insert(sand);
                rested += 1;
                break 'sand;
            }
            for cand in [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ] {
                if !grid.contains(&cand) {
                    // println!("{sand:?} -> {cand:?}");
                    sand = cand;
                    continue 'sand;
                }
            }
            grid.insert(sand);
            rested += 1;
            break;
        }
    }

    println!("{rested}");
}

fn p(s: &str) -> Pos {
    let (a, b) = s.split_once(',').unwrap();
    (i(a), i(b))
}

fn i(s: &str) -> i16 {
    s.parse().unwrap()
}
