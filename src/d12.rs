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

    println!("{}", search(&grid, (start, end), &mut HashMap::new()));
}

fn search(grid: &[Vec<u8>], (start, end): (Pos, Pos), memo: &mut HashMap<Pos, usize>) -> usize {
    if let Some(result) = memo.get(&start) {
        return *result;
    }
    let h = grid.len();
    let w = grid[0].len();

    if start == end {
        return 0;
    }

    let here = grid[start.1][start.0];

    // println!("visiting {start:?}");

    let mut opts = Vec::new();
    for mv in [
        (start.0 + 1, start.1),
        (start.0.checked_sub(1).unwrap_or(usize::MAX), start.1),
        (start.0, start.1 + 1),
        (start.0, start.1.checked_sub(1).unwrap_or(usize::MAX)),
    ] {
        if mv.0 >= w || mv.1 >= h {
            continue;
        }
        let prop = grid[mv.1][mv.0];
        if prop > here + 1 {
            continue;
        }
        let res = search(grid, (mv, end), memo);
        // println!("{start:?} {here} ({}) -> {mv:?} -> {result}", ('a' as u8 + here) as char);
        memo.insert(mv, res);

        opts.push(res);
    }

    let res = opts
        .into_iter()
        .min()
        .and_then(|x| x.checked_add(1))
        .unwrap_or(usize::MAX);
    memo.insert(start, res);
    res
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
