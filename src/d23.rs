use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (i64, i64);

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    S,
    W,
    E,
}

use Dir::*;

pub fn solve() {
    let mut grid: HashSet<Pos> = include_str!("d23.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                (
                    (x as i64, y as i64),
                    match c {
                        '#' => true,
                        '.' => false,
                        c => unreachable!("{c:?}"),
                    },
                )
            })
        })
        .filter_map(|(p, v)| if v { Some(p) } else { None })
        .collect();

    let mut moves = VecDeque::from([N, S, W, E]);

    for round in 0..10 {
        println!();
        println!("{round} {moves:?}");

        let mut proposals = HashMap::with_capacity(grid.len());
        'elf: for elf in &grid {
            let mut has_neighbour = false;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let cand: Pos = (elf.0 + dx, elf.1 + dy);
                    has_neighbour |= grid.contains(&cand);
                }
            }
            if !has_neighbour {
                println!("{elf:?} has no neighbour");
                continue;
            }

            'mov: for mov in &moves {
                let blocks = match mov {
                    N => [(-1, -1), (0, -1), (1, -1)],
                    S => [(-1, 1), (0, 1), (1, 1)],
                    E => [(1, -1), (1, 0), (1, 1)],
                    W => [(-1, -1), (-1, 0), (-1, 1)],
                };
                for block in blocks {
                    let block = (elf.0 + block.0, elf.1 + block.1);
                    if grid.contains(&block) {
                        // println!("{elf:?} can't move {mov:?} 'cos it's blocked by {block:?}");
                        continue 'mov;
                    }
                }
                let want = blocks[1];
                let cand = (elf.0 + want.0, elf.1 + want.1);
                // println!("{elf:?} moving {mov:?} to {cand:?}");
                proposals
                    .entry(cand)
                    .or_insert(Vec::with_capacity(4))
                    .push(*elf);
                continue 'elf;
            }
        }

        for (cand, who) in proposals {
            if who.len() > 1 {
                continue;
            }
            let who = who[0];
            grid.remove(&who);
            assert!(grid.insert(cand));
        }
        moves.rotate_left(1);
        for y in -2..10 {
            for x in -3..11 {
                print!("{}", if grid.contains(&(x, y)) { "#" } else { "." });
            }
            println!();
        }
        println!("{:?}", grid.iter().sorted().collect_vec());

        // break;
    }

    let MinMaxResult::MinMax(l, r) = grid.iter().map(|(x, _)| *x).minmax() else { unreachable!() };
    let MinMaxResult::MinMax(t, b) = grid.iter().map(|(_, y)| *y).minmax() else { unreachable!() };
    println!(
        "{:?} {:?} {} {}",
        (t, l),
        (b, r),
        (r - l + 1) * (b - t + 1),
        grid.len()
    );
}
