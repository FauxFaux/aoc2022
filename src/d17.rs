use itertools::Itertools;

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
    let mut rights = rights.into_iter().cycle();

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
    let mut pieces = pieces.into_iter().cycle();

    let mut grid: Vec<[bool; W]> = Vec::with_capacity(9000);
    let mut top = 0;

    // grid: 0 is floor, len is height
    // piece: 0,0 is top left, so coordinates are upside down?
    for _ in 0..=2022 {
        while grid.len() < top + 10 {
            grid.push(Default::default());
        }

        for i in top.. {
            if grid[i].iter().all(|x| !x) {
                top = i;
                #[cfg(debug_assertions)]
                println!("top is now {top}");
                break;
            }
        }

        #[cfg(debug_assertions)]
        for line in grid.iter().rev() {
            for c in line {
                print!("{}", if *c { "#" } else { "." });
            }
            println!();
        }
        #[cfg(debug_assertions)]
        println!();

        let piece = pieces.next().unwrap();
        let mut us: Pos = (2, top + 2 + piece.height);
        loop {
            let dir;
            let cand = if rights.next().unwrap() {
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
                #[cfg(debug_assertions)]
                println!("{us:?} *did* move down");
                us = cand;
            } else {
                freeze(&mut grid, &piece, us);
                break;
            }
        }
    }
    println!("{top}");
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
