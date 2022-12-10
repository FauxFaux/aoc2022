use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() {
    let cmds = include_str!("d10.txt")
        .lines()
        .map(|l| l.split_once(' ').map(|(_, c)| p(c)))
        .collect_vec();

    println!("{cmds:?}");

    let mut sum = 0;

    let mut check = |pc, x| match pc {
        20 | 60 | 100 | 140 | 180 | 220 => {
            sum += pc * x;
            println!("{pc} {x}");
        }
        _ => (),
    };

    let mut x = 1;
    let mut pc = 0;

    for cmd in cmds {
        match cmd {
            None => {
                pc += 1;
                check(pc, x);
            }
            Some(v) => {
                pc += 1;
                check(pc, x);
                pc += 1;
                check(pc, x);
                x += v;
            }
        }
    }

    println!("{sum}");
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
