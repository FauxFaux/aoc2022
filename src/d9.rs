use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coord = (i64, i64);

pub fn solve() {
    let cmds = include_str!("d9.txt")
        .lines()
        .map(|l| l.split_once(' ').expect("lines"))
        .flat_map(|(d, c)| (0..p(c)).map(|_| d.chars().next().unwrap()))
        .collect_vec();

    println!("{cmds:?}");

    let mut visited = HashSet::<Coord>::new();
    let mut head: Coord = (0, 0);
    let mut tails: Vec<Coord> = vec![(0, 0); 9];
    for cmd in cmds {
        match cmd {
            'L' => head.0 -= 1,
            'R' => head.0 += 1,
            'U' => head.1 -= 1,
            'D' => head.1 += 1,
            other => unreachable!("move: {other:?}"),
        }
        mv(head, &mut tails[0]);
        for (i, (targ, tail)) in (0..9).tuple_windows().enumerate() {
            mv(tails[targ], &mut tails[tail]);
            println!("{cmd} {i} targ: {targ:?}, tail: {tail:?}");
        }

        visited.insert(*tails.last().unwrap());
    }

    println!("{visited:?}");
    println!("{}", visited.len());
}

fn mv(targ: Coord, tail: &mut Coord) {
    let pick = match (targ.0 - tail.0, targ.1 - tail.1) {
        (x, y) if x.abs() <= 1 && y.abs() <= 1 => (0, 0),
        (0, y) if y > 0 => (0, 1),
        (0, y) if y < 0 => (0, -1),
        (x, 0) if x < 0 => (-1, 0),
        (x, 0) if x > 0 => (1, 0),
        (x, y) if x > 0 && y > 0 => (1, 1),
        (x, y) if x < 0 && y < 0 => (-1, -1),
        (x, y) if x > 0 && y < 0 => (1, -1),
        (x, y) if x < 0 && y > 0 => (-1, 1),
        other => unreachable!("tail: {other:?}"),
    };

    tail.0 += pick.0;
    tail.1 += pick.1;
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
