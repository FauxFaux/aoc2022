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

    let (el, _, er) = op.remove("root").unwrap();
    num.remove("humn").unwrap();

    for _ in 0..1000 {
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

    fn render(
        (num, op): (&HashMap<Name, i64>, &HashMap<Name, (Name, char, Name)>),
        name: &Name,
    ) -> String {
        if let Some(n) = num.get(name) {
            return format!("{n}");
        }
        match op.get(name) {
            Some((l, o, r)) => format!("({}{o}{})", render((num, op), l), render((num, op), r)),
            _ => format!("x"),
        }
    }

    println!("{}={}", render((&num, &op), &el), render((&num, &op), &er));
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
