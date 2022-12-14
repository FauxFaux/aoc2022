use itertools::Itertools;
use std::collections::HashMap;

type Pos = (i64, i64);

pub fn solve() {
    let lines = include_str!("d14.txt")
        .lines()
        .map(|x| x.split(" -> ").map(p).collect_vec())
        .collect_vec();

    let mut grid = HashMap::<Pos, bool>::new();
    for line in &lines {
        let mut cursor = line[0];
        grid.insert(cursor, true);
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
                grid.insert(cursor, true);
            }
        }
    }

    let bottom = *grid.keys().map(|(_, y)| y).max().unwrap();
    let mut rested = 0usize;

    'sim: while !grid.get(&(500, 0)).copied().unwrap_or_default() {
        let mut sand: Pos = (500, 0);
        'sand: loop {
            if sand.1 == bottom + 1 {
                grid.insert(sand, true);
                // println!("baselining: {sand:?}");
                rested += 1;
                break 'sand;
            }
            for cand in [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ] {
                if !*grid.get(&cand).unwrap_or(&false) {
                    // println!("{sand:?} -> {cand:?}");
                    sand = cand;
                    continue 'sand;
                }
            }
            grid.insert(sand, true);
            rested += 1;
            break;
        }
    }

    println!("{rested}");
}

fn p(s: &str) -> (i64, i64) {
    let (a, b) = s.split_once(',').unwrap();
    (i(a), i(b))
}

fn i(s: &str) -> i64 {
    s.parse().unwrap()
}
