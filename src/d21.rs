use itertools::{Either, Itertools};
use std::collections::HashMap;

type Name = &'static str;

// #[derive(Copy, Clone, Debug)]
// enum Monkey {
//     Num(i64),
//     Op((Name, char, Name)),
// }

pub fn solve() {
    let (mut num, mut op): (HashMap<Name, i64>, HashMap<Name, (Name, char, Name)>) =
        include_str!("d21.txt").lines().partition_map(|l| {
            let parts = l
                .split(|c: char| c.is_whitespace() || c == ':')
                .collect_vec();
            match parts.len() {
                3 => Either::Left((parts[0], p(parts[2]))),
                5 => Either::Right((
                    parts[0],
                    (parts[2], parts[3].chars().next().unwrap(), parts[4]),
                )),
                o => unreachable!("{o}"),
            }
        });

    while !num.contains_key("root") {
        op.retain(|name, (l, o, r)| match (num.get(l), num.get(r)) {
            (Some(l), Some(r)) => {
                num.insert(
                    name,
                    match o {
                        '+' => *l + *r,
                        '-' => *l - *r,
                        '*' => *l * *r,
                        '/' => *l / *r,
                        _ => unreachable!("{o}"),
                    },
                );
                false
            }
            _ => true,
        });
    }

    println!("{num:?} {op:?}");
    dbg!(num["root"]);
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
