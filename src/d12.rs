use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);

pub fn solve() {
    let grid = include_str!("d12.txt")
        .trim()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let start = find(&grid, 'S');
    let end = find(&grid, 'E');

    let grid = grid
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|c| match c {
                    'S' => 0u8,
                    'E' => 25,
                    o => o as u8 - b'a',
                })
                .collect_vec()
        })
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();
    let mut best = HashMap::new();
    let mut queue = Vec::new();
    best.insert(start, 0usize);
    queue.push(start);
    'outer: for depth in 0usize.. {
        let now = queue.clone();
        queue.clear();
        println!("{depth} {now:?}");
        assert!(!now.is_empty());
        for loc in now {
            if loc == end {
                break 'outer;
            }

            let here = grid[loc.1][loc.0];
            for mv in [
                (loc.0 + 1, loc.1),
                (loc.0.checked_sub(1).unwrap_or(usize::MAX), loc.1),
                (loc.0, loc.1 + 1),
                (loc.0, loc.1.checked_sub(1).unwrap_or(usize::MAX)),
            ] {
                if mv.0 >= w || mv.1 >= h {
                    continue;
                }
                let prop = grid[mv.1][mv.0];
                if prop > here + 1 {
                    continue;
                }

                if best.contains_key(&mv) {
                    println!("can't move {mv:?} as already visited");
                    continue;
                }
                best.insert(mv, depth);
                queue.push(mv);
            }
        }
    }
}

fn find(grid: &[Vec<char>], c: char) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == c {
                return (x, y);
            }
        }
    }
    panic!("not found: {c}")
}

fn c(v: u8) -> char {
    ('a' as u8 + v) as char
}
