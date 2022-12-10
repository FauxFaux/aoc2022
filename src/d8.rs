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
            let mut visible = 1;
            visible *= m((0..x).rev(), |ix| g[y][ix] >= us);
            visible *= m(x + 1..r, |ix| g[y][ix] >= us);

            visible *= m((0..y).rev(), |iy| g[iy][x] >= us);
            visible *= m(y + 1..b, |iy| g[iy][x] >= us);

            println!("{x} {y} ({us}): {visible}");
            sum = sum.max(visible);
        }
    }
    println!("{sum}");
}

fn m(
    range: impl Iterator<Item = usize> + ExactSizeIterator + Clone,
    sub: impl FnMut(usize) -> bool,
) -> usize {
    range
        .clone()
        .position(sub)
        .map(|x| x + 1)
        .unwrap_or(range.len())
}
