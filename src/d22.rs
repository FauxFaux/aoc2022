use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Cmd {
    TurnLeft,
    TurnRight,
    Move(usize),
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
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

use Cmd::*;

type Pos = (usize, usize);

pub fn solve() {
    let mut lines = include_str!("d22.txt").lines().collect_vec();
    let instructions = lines.pop().unwrap();
    let cmds = instructions
        .split_inclusive(|c: char| !c.is_ascii_digit())
        .flat_map(|s| {
            if let Some(s) = s.strip_suffix('L') {
                vec![Move(p(s)), TurnLeft]
            } else if let Some(s) = s.strip_suffix('R') {
                vec![Move(p(s)), TurnRight]
            } else {
                vec![Move(p(s))]
            }
        })
        .collect_vec();

    let grid = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    ' ' => None,
                    _ => unreachable!("{c}"),
                })
                .collect_vec()
        })
        .collect_vec();

    println!("{cmds:?}");

    let mut here: Pos = (grid[0].iter().position(|&x| x == Some(false)).unwrap(), 0);
    let mut face = Dir::E;

    let max_x = grid[0].len() as i64;
    let max_y = grid.len() as i64;

    for cmd in cmds {
        match cmd {
            Move(dist) => {
                println!("{here:?}, {dist} to move, {face:?}");
                let (dx, dy) = face.diff();
                'mov: for step in 0..dist {
                    let (mut cx, mut cy) = (here.0 as i64, here.1 as i64);
                    'wrap: loop {
                        cx = (cx + dx + max_x) % max_x;
                        cy = (cy + dy + max_y) % max_y;
                        match grid[cy as usize]
                            .get(cx as usize)
                            .copied()
                            .unwrap_or_default()
                        {
                            None => (),
                            Some(false) => break 'wrap,
                            Some(true) => break 'mov,
                        }
                        // println!("{:?} + {:?} is still inside a void", (cx, cy), (dx, dy));
                    }
                    println!("{step}: stepped from {here:?} to {:?}", (cx, cy));
                    here = (cx as usize, cy as usize);
                }
            }
            TurnLeft => {
                face = match face {
                    Dir::N => Dir::W,
                    Dir::W => Dir::S,
                    Dir::S => Dir::E,
                    Dir::E => Dir::N,
                };
            }
            TurnRight => {
                face = match face {
                    Dir::N => Dir::E,
                    Dir::E => Dir::S,
                    Dir::S => Dir::W,
                    Dir::W => Dir::N,
                };
            }
        }
    }

    println!("{here:?} {face:?}");
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
