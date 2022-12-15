use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

type Pos = (i64, i64);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Ent {
    Sensor,
    Beacon,
    Excluded,
}

pub fn solve() {
    let lines = include_str!("d15.txt")
        .lines()
        .map(|x| {
            x.split(|c| matches!(c, ',' | '=' | ':'))
                .tuples()
                .map(|(_s, sx, _e, sy, _b, bx, _eo, by)| ((p(sx), p(sy)), (p(bx), p(by))))
                .next()
                .unwrap()
        })
        .collect_vec();

    let mut grid = HashMap::<Pos, Ent>::new();

    for (s, b) in lines.iter().copied() {
        grid.insert(s, Ent::Sensor);
        grid.insert(b, Ent::Beacon);
        let r = d(s, b);
        println!("{s:?} {b:?} {r}");
        for x in (s.0 - r)..=s.0 + r {
            let y = 2000000;
            // for y in s.1-r..=s.1+r {
            //     if y != 2000000 { continue }
            if d(s, (x, y)) <= r {
                grid.entry((x, y)).or_insert(Ent::Excluded);
            }
            // }
        }
    }

    // println!("{lines:?}");
    // println!("{grid:?}");

    let cnt = grid
        .iter()
        .filter(|((_, y), e)| *y == 2000000 && **e == Ent::Excluded)
        .count();

    println!("{}", cnt);
}

fn d(a: Pos, b: Pos) -> i64 {
    (max(a.0, b.0) - min(a.0, b.0)) + (max(a.1, b.1) - min(a.1, b.1))
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
