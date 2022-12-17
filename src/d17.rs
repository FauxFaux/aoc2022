use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

type Pos = (usize, usize);

const W: usize = 7;

pub fn solve() {
    let rights = include_str!("d17.txt")
        .trim()
        .chars()
        .map(|c| match c {
            '<' => false,
            '>' => true,
            other => unreachable!("{other:?}"),
        })
        .collect_vec();
    let mut rights = rights.into_iter().enumerate().cycle();

    let pieces = [
        r#"
####
    "#,
        r#"
.#.
###
.#."#,
        r#"
..#
..#
###
"#,
        r#"
#
#
#
#"#,
        r#"
##
##
"#,
    ];

    let pieces = pieces.iter().map(|s| piece(s)).collect_vec();
    let mut pieces = pieces.into_iter().enumerate().cycle();

    let mut grid: Vec<[bool; W]> = Vec::with_capacity(9000);
    let mut top = 0;

    let mut memo = HashMap::with_capacity(9000);

    let mut done = false;
    // grid: 0 is floor, len is height
    // piece: 0,0 is top left, so coordinates are upside down?
    for round in 0..=1000000000000u64 {
        while grid.len() < top + 10 {
            grid.push(Default::default());
        }

        #[cfg(debug_assertions)]
        print(&grid[grid.len() - 10..]);

        let (pn, piece) = pieces.next().unwrap();
        let mut us: Pos = (2, top + 2 + piece.height);
        loop {
            let dir;
            let (mn, go_right) = rights.next().unwrap();
            let cand = if go_right {
                dir = "right";
                (us.0 + 1, us.1)
            } else {
                dir = "left";
                (us.0.saturating_sub(1), us.1)
            };

            if happy(&grid, &piece, cand) {
                #[cfg(debug_assertions)]
                println!("{us:?} *did* move {dir}");
                us = cand;
            }

            let cand = (us.0, us.1.saturating_sub(1));
            if us.1 != 0 && happy(&grid, &piece, cand) {
                // #[cfg(debug_assertions)]
                // println!("{us:?} *did* move down");
                us = cand;
            } else {
                println!("{}", round as f64 / 1000000000000.);
                freeze(&mut grid, &piece, us);
                for i in top.. {
                    if grid[i].iter().all(|x| !x) {
                        top = i;
                        break;
                    }
                }
                let mut filled = [false; 7];
                let mut i = top;
                loop {
                    for (px, p) in grid[i].iter().enumerate() {
                        filled[px] |= *p;
                    }
                    if filled.iter().all(|x| *x) {
                        break;
                    }

                    if i == 0 {
                        break;
                    }

                    i -= 1;
                }

                let head = grid[i..top].to_vec();
                let key = (mn, pn, head);
                match memo.entry(key) {
                    Entry::Occupied(en) => {
                        println!("{round} {top} {:?}", en.get());
                        done = true;
                        // break 'game;
                    }
                    Entry::Vacant(hole) => {
                        hole.insert((round, top));
                    }
                }

                // off by one from example
                // In [21]: 1514285714288 - (int(999999999973/35)*(102-49)+49 + (132-102))
                // Out[21]: -1

                // extra rounds
                // In [25]: ((1000000000000-84) % (1824 - 84))
                // Out[25]: 1096

                //      goal  - init top  / loop len       * loop height + init height + (height from extra rounds)
                // int((1000000000000-84) / (1824 - 84))   * (2900-141)  + 141         + (4638 - 2900) - 1

                if done {
                    println!("{round} {top}")
                }
                // print(&head);
                // println!("top {top} -> {i} are dumb");

                break;
            }
        }
    }
    println!("{top}");
}

fn print(grid: &[[bool; 7]]) {
    for line in grid.iter().rev() {
        for c in line {
            print!("{}", if *c { "#" } else { "." });
        }
        println!();
    }
}

fn happy(grid: &Vec<[bool; 7]>, piece: &Piece, (px, py): (usize, usize)) -> bool {
    for (dy, line) in piece.inner.iter().copied().enumerate() {
        for (dx, point) in line.into_iter().enumerate() {
            if !point {
                continue;
            }

            if px + dx >= W {
                // println!("({dy}, {dx}) hits a wall");
                return false;
            }

            // the floor
            if dy > py {
                return false;
            }

            if grid[py - dy][px + dx] {
                return false;
            }
        }
    }

    true
}

fn freeze(grid: &mut Vec<[bool; 7]>, piece: &Piece, (px, py): (usize, usize)) {
    for (dy, line) in piece.inner.iter().copied().enumerate() {
        for (dx, point) in line.into_iter().enumerate() {
            if !point {
                continue;
            }

            grid[py - dy][px + dx] = true;
        }
    }
}

#[derive(Copy, Clone)]
struct Piece {
    inner: [[bool; 4]; 4],
    height: usize,
}

fn piece(s: &str) -> Piece {
    let mut grid = Vec::new();
    for line in s.trim().lines() {
        let mut pixel = Vec::new();
        for c in line.trim().chars() {
            pixel.push(match c {
                '#' => true,
                '.' => false,
                _ => unreachable!("{c:?}"),
            });
        }
        while pixel.len() != 4 {
            pixel.push(false);
        }

        grid.push(pixel.try_into().unwrap());
    }
    let height = grid.len();
    while grid.len() != 4 {
        grid.push([false, false, false, false]);
    }
    Piece {
        inner: grid.try_into().unwrap(),
        height,
    }
}
