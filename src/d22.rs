#![allow(warnings, unused)]

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Cmd {
    TurnLeft,
    TurnRight,
    Move(usize),
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    fn flip(&self) -> Self {
        use Dir::*;
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
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
    use Dir::*;
    let trans = |face: u8, dir: Dir| match (face, dir) {
        (0, N) => (5, E),
        (0, E) => (1, E),
        (0, S) => (2, S),
        (0, W) => (3, E),
        (1, N) => (5, N),
        (1, E) => (4, W),
        (1, S) => (2, W),
        (1, W) => (0, W),
        (2, N) => (0, N),
        (2, E) => (1, N),
        (2, S) => (4, S),
        (2, W) => (3, S),
        (3, N) => (2, E),
        (3, E) => (4, E),
        (3, S) => (5, S),
        (3, W) => (0, E),
        (4, N) => (2, N),
        (4, E) => (1, W),
        (4, S) => (5, W),
        (4, W) => (3, W),
        (5, N) => (3, N),
        (5, E) => (4, N),
        (5, S) => (1, S),
        (5, W) => (0, S),
        other => unreachable!("{other:?}"),
    };

    for face in 0..6 {
        for dir in [N, S, E, W] {
            let (nf, nd) = trans(face, dir);
            assert_eq!((face, dir.flip()), trans(nf, nd.flip()));
        }
    }

    // println!("{cmds:?}");

    let mut face = 0;
    let mut here: Pos = (faces[0][0].iter().position(|&x| x == false).unwrap(), 0);
    let mut heading = E;

    let max_x = faces[0][0].len() as i64;
    let max_y = faces[0].len() as i64;

    let to_map = |face: u8, (x, y): (usize, usize)| {
        let (dx, dy) = match face {
            0 => (50, 0),
            1 => (100, 0),
            2 => (50, 50),
            3 => (0, 100),
            4 => (50, 100),
            5 => (0, 150),
            o => unreachable!("{o}"),
        };
        (x + dx, y + dy)
    };

    for (i, cmd) in cmds.iter().enumerate() {
        match *cmd {
            Move(dist) => {
                // println!("{i}: {:?}@{face:?}, {heading:?}, {dist} to move", to_map(face, here));
                println!(
                    "{i}: {:?}, {}, {dist} to move       || {here:?}@{face:?}",
                    to_map(face, here),
                    match heading {
                        E => 0,
                        S => 1,
                        W => 2,
                        N => 3,
                    }
                );
                'mov: for step in 0..dist {
                    let (mut cx, mut cy) = (here.0 as i64, here.1 as i64);
                    let mut cf = face;
                    let mut ch = heading;
                    'wrap: loop {
                        let (dx, dy) = ch.diff();
                        cx += dx;
                        cy += dy;
                        if cx >= max_x {
                            (cf, ch) = trans(cf, ch);
                            cx = 0;
                        }
                        if cy >= max_y {
                            (cf, ch) = trans(cf, ch);
                            cy = 0;
                        }
                        if cx < 0 {
                            (cf, ch) = trans(cf, ch);
                            cx = max_x - 1;
                        }
                        if cy < 0 {
                            (cf, ch) = trans(cf, ch);
                            cy = max_y - 1;
                        }
                        if i == 384 {
                            println!("step: {:?}@{cf}, {ch:?}", (cx, cy));
                            // print(&faces[cf as usize], (cx as usize, cy as usize));
                        }
                        match faces[cf as usize][cy as usize][cx as usize] {
                            false => break 'wrap,
                            true => {
                                println!("{:?}@{cf}, {ch:?}: wall", (cx, cy));
                                // print(&faces[face as usize], here);

                                break 'mov;
                            }
                        }
                    }
                    if cf != face {
                        // print(&faces[face as usize], here);
                        println!(
                            "{here:?}@{face}, {heading:?} to {:?}@{cf}, {ch:?}",
                            (cx, cy)
                        );
                        // print(&faces[cf as usize], (cx as usize, cy as usize));
                    }
                    here = (cx as usize, cy as usize);
                    face = cf;
                    heading = ch;
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

    // 39185 too low
    // 137058 too low
    println!("{face} {here:?} {heading:?}");
}

fn print(face: &[[bool; 50]], p: (usize, usize)) {
    for (gy, row) in face.iter().enumerate() {
        for (gx, column) in row.iter().enumerate() {
            if (gx, gy) == p {
                print!("X");
            } else {
                print!("{}", if *column { '.' } else { '_' });
            }
        }
        println!();
    }
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
