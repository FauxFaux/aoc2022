use itertools::Itertools;

pub fn solve() {
    let g: Vec<Vec<u8>> = include_str!("d8.txt")
        .lines()
        .map(|l| l.chars().map(|c| (c as u8 - b'0') as u8).collect_vec())
        .collect_vec();
    println!("{g:?}");
    let r = g[0].len();
    let b = g.len();
    let mut sum = 0;
    for y in 0..b {
        for x in 0..r {
            let us = g[y][x];
            if x == 0 || y == 0 || x == r - 1 || y == b - 1 {}
            let mut visible = false;
            visible |= (0..x).all(|ix| g[y][ix] < us);
            visible |= (x + 1..r).all(|ix| g[y][ix] < us);

            visible |= (0..y).all(|iy| g[iy][x] < us);
            visible |= (y + 1..b).all(|iy| g[iy][x] < us);

            println!("{x} {y} ({us}): {visible}");
            sum += if visible { 1 } else { 0 };
        }
    }
    println!("{sum}");
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
