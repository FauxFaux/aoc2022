use itertools::Itertools;
use std::cmp::{max, min};

type Pos = (i64, i64);

pub fn solve() {
    let lines = include_str!("d15.txt")
        .lines()
        .map(|x| {
            x.split(|c| matches!(c, ',' | '=' | ':'))
                .tuples()
                .map(|(_s, sx, _e, sy, _b, bx, _eo, by)| ((p(sx), p(sy)), (p(bx), p(by))))
                .map(|(s, b)| (s, d(s, b)))
                .next()
                .unwrap()
        })
        .collect_vec();

    let mx = 4000000;

    for y in 0..mx {
        let mut ranges = Vec::new();
        for (s, r) in lines.iter().copied() {
            let yo = max(s.1, y) - min(s.1, y);
            if yo > r {
                continue;
            }
            let w = r - yo;

            ranges.push((max(s.0 - w, 0), min(s.0 + w, mx)));
        }
        ranges.sort_by_key(|x| x.0);

        for (es, e) in &ranges {
            for (s, se) in &ranges {
                let c = e + 1;
                if c + 1 != *s {
                    continue;
                }
                let mut bad = false;
                for (s, e) in &ranges {
                    if c >= *s && c <= *e {
                        bad = true;
                    }
                }
                if !bad {
                    println!("{c} {y} {s}-{se} {es}-{e} {:?}", ranges);
                }
            }
        }
    }
}

fn d(a: Pos, b: Pos) -> i64 {
    (max(a.0, b.0) - min(a.0, b.0)) + (max(a.1, b.1) - min(a.1, b.1))
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
