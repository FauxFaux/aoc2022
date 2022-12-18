use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() {
    let mut pts: HashSet<[i8; 3]> = include_str!("d18.txt")
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

    let moves = [
        [1, 0, 0],
        [-1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, 1],
        [0, 0, -1],
    ];

    for x in 0..20i8 {
        for y in 0..20 {
            for z in 0..20 {
                if pts.contains(&[x, y, z]) {
                    continue;
                }
                let mut escaped = false;
                let mut filled = pts.clone();
                let mut stack = vec![[x, y, z]];
                filled.insert([x, y, z]);
                while let Some([x, y, z]) = stack.pop() {
                    for [dx, dy, dz] in moves {
                        let cand = [x + dx, y + dy, z + dz];
                        if filled.insert(cand) {
                            stack.push(cand);
                        }
                    }
                    if x > 20 || x < 0 || y > 20 || y < 0 || z > 20 || z < 0 {
                        escaped = true;
                        break;
                    }
                }
                if !escaped {
                    pts = filled;
                    // println!("{x} {y} {z} {}", filled.len());
                }
            }
        }
    }

    let mut free = 0usize;
    for pt in &pts {
        let [x, y, z] = *pt;
        for [dx, dy, dz] in moves {
            if pts.contains(&[x + dx, y + dy, z + dz]) {
                continue;
            }
            free += 1;
        }
    }

    println!("{free:?}")
}
