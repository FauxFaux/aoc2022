use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Pos = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn diff(&self) -> (i64, i64) {
        use Dir::*;
        match self {
            N => (0, -1),
            S => (0, 1),
            E => (1, 0),
            W => (-1, 0),
        }
    }
}

use Dir::*;

type Storms = Vec<(Pos, Dir)>;

pub fn solve() {
    let mut lines = include_str!("d24.txt").trim().lines().collect_vec();
    let head = lines.remove(0);
    let tail = lines.pop().unwrap();

    let (w, h) = (head.len() - 2, lines.len());

    let start: Pos = (head.chars().position(|c: char| c == '.').unwrap(), 0);
    let end: Pos = (tail.chars().position(|c: char| c == '.').unwrap(), h + 1);

    let storms: Storms = lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim_matches('#')
                .chars()
                .enumerate()
                .flat_map(|(x, c)| {
                    Some((
                        (1 + x, 1 + y),
                        match c {
                            '<' => W,
                            '>' => E,
                            '^' => N,
                            'v' => S,
                            '.' => return None,
                            o => unimplemented!("{o:?}"),
                        },
                    ))
                })
                // BORROW CHECKER
                .collect_vec()
        })
        .collect_vec();

    let mut grids = Vec::new();
    let mut storms = storms;

    for s in 0..w * h {
        println!("step {s}/{}", w * h);
        // print((w, h), &storms);
        let mut grid = vec![vec![true; w + 2]; h + 3];
        // lol
        for y in 0..h + 3 {
            for x in 0..w + 2 {
                if x == 0 || y == 0 || x == w + 1 || y >= h + 1 {
                    grid[y][x] = false;
                }
            }
        }

        grid[start.1][start.0] = true;
        grid[end.1][end.0] = true;

        for (pos, _dir) in &storms {
            grid[pos.1][pos.0] = false;
        }

        // println!("{grid:?}");
        // print((w, h), &storms);

        grids.push(grid);
        storms = step((w, h), &storms);

        // println!();
    }

    let one = search((start, 0), end, &grids);
    println!("{one}");
    let two = search((end, one), start, &grids);
    let three = search((start, two), end, &grids);
    println!("{three}");
}

fn search(start: (Pos, usize), end: Pos, grids: &Vec<Vec<Vec<bool>>>) -> usize {
    let mut visited: HashSet<(Pos, usize)> = HashSet::with_capacity(9000);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(((ix, iy), movno)) = queue.pop_front() {
        if (ix, iy) == end {
            return movno;
        }
        let newno = (movno + 1) % grids.len();
        let grid = &grids[newno];
        for cand in [
            (ix, iy),
            (ix - 1, iy),
            (ix + 1, iy),
            (ix, iy.saturating_sub(1)),
            (ix, iy + 1),
        ] {
            if grid[cand.1][cand.0] {
                if visited.insert((cand, newno)) {
                    queue.push_back((cand, movno + 1));
                }
            }
        }
    }
    unreachable!()
}

fn print((w, h): (usize, usize), storms: &Storms) {
    for y in 0..h + 2 {
        for x in 0..w + 2 {
            let mut count = 0;
            for (sp, _) in storms {
                if *sp == (x, y) {
                    count += 1;
                }
            }
            if count == 0 {
                print!(".");
            } else {
                print!("{count}");
            }
        }
        println!();
    }
}

fn step((w, h): (usize, usize), storms: &Storms) -> Storms {
    let mut new = Storms::with_capacity(storms.len());
    for (pos, dir) in storms {
        let (dx, dy) = dir.diff();
        let (cx, cy) = (
            usize::try_from(pos.0 as i64 + dx).unwrap(),
            usize::try_from(pos.1 as i64 + dy).unwrap(),
        );
        let cx = if cx == 0 {
            w
        } else if cx > w {
            1
        } else {
            cx
        };

        let cy = if cy == 0 {
            h
        } else if cy > h {
            1
        } else {
            cy
        };

        new.push(((cx, cy), *dir));
    }
    new
}
