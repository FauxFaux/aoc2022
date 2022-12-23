#![allow(warnings, unused)]

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

    const CS: usize = 50;

    let mut faces = Vec::new();

    for outer in grid.array_chunks::<CS>() {
        // what even is this, itertools
        let mut parts = vec![Vec::new(); 8];
        for line in outer {
            for (pos, inner) in line.array_chunks::<CS>().enumerate() {
                parts[pos].push(inner);
            }
        }

        faces.extend(
            parts
                .into_iter()
                .filter(|x| !x.is_empty())
                .filter(|x| x[0][0].is_some())
                .map(|x| x.into_iter().map(|x| x.map(|x| x.unwrap())).collect_vec()),
        );
    }

    assert_eq!(6, faces.len());

    /// .01
    /// .2.
    /// 34.
    /// 5..
    /// https://quad.pe/e/ywZgQqDgMQ.png
    /// back: 0 (top points towards the top)
    /// right: 1 (top points towards the top)
    /// bottom: 2 (top points towards the back)
    /// left: 3 (top points towards the bottom)
    /// front: 4 (top points towards the bottom)
    /// top: 5 (top points towards the left)
    use Dir::*;
    let trans = |face: u8, dir: Dir| match (face, dir) {
        (2, N) => (0, N),
        other => unreachable!("{other:?}"),
    };

    println!("{cmds:?}");

    let mut face = 0;
    let mut grid = &faces[0];
    let mut here: Pos = (grid[0].iter().position(|&x| x == false).unwrap(), 0);
    let mut heading = E;

    let max_x = faces[0][0].len() as i64;
    let max_y = faces[0].len() as i64;

    for cmd in cmds {
        match cmd {
            Move(dist) => {
                println!("{here:?}, {dist} to move, {face:?}");
                let (dx, dy) = heading.diff();
                'mov: for step in 0..dist {
                    let (mut cx, mut cy) = (here.0 as i64, here.1 as i64);
                    'wrap: loop {
                        cx += dx;
                        cy += dy;
                        if cx > max_x {
                            (face, heading) = trans(face, heading);
                            cx = 0;
                        }
                        if cy > max_y {
                            (face, heading) = trans(face, heading);
                            cy = 0;
                        }
                        if cx < 0 {
                            (face, heading) = trans(face, heading);
                            cx = max_x;
                        }
                        if cy < 0 {
                            (face, heading) = trans(face, heading);
                            cy = max_y;
                        }
                        grid = &faces[face as usize];
                        match grid[cy as usize][cx as usize] {
                            false => (),
                            true => break 'mov,
                        }
                        // println!("{:?} + {:?} is still inside a void", (cx, cy), (dx, dy));
                    }
                    println!("{step}: stepped from {here:?} to {:?}", (cx, cy));
                    here = (cx as usize, cy as usize);
                }
            }
            TurnLeft => {
                heading = match heading {
                    Dir::N => Dir::W,
                    Dir::W => Dir::S,
                    Dir::S => Dir::E,
                    Dir::E => Dir::N,
                };
            }
            TurnRight => {
                heading = match heading {
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
