use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() {
    let pts: HashSet<[i8; 3]> = include_str!("d18.txt")
        .trim()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<i8>().unwrap())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut free = 0usize;
    for pt in &pts {
        let [x, y, z] = *pt;
        for [dx, dy, dz] in [
            [1, 0, 0],
            [-1, 0, 0],
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, 1],
            [0, 0, -1],
        ] {
            if pts.contains(&[x + dx, y + dy, z + dz]) {
                continue;
            }
            free += 1;
        }
    }

    println!("{free:?}")
}
