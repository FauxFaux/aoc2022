use itertools::{Either, Itertools};
use num_integer::Integer;
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

    println!("{}={}", render((&num, &op), &el), render((&num, &op), &er));

    let mut targ = num[er];
    let mut here = el;

    loop {
        let (l, o, r) = op[here];
        println!("({l}: {:?}) {o} ({r}: {:?})", num.get(l), num.get(r));
        match (num.get(l), num.get(r)) {
            (Some(l), None) => {
                match o {
                    '*' => {
                        let (d, r) = targ.div_rem(l);
                        assert_eq!(r, 0);
                        targ = d;
                    }
                    '+' => {
                        targ -= *l;
                    }
                    '-' => {
                        targ = *l - targ;
                    }
                    o => todo!("l {o:?}"),
                }
                println!("{} = {}", render((&num, &op), &r), targ);
                here = r;
            }
            (None, Some(r)) => {
                match o {
                    '/' => {
                        targ *= *r;
                    }
                    '-' => {
                        targ += *r;
                    }
                    '+' => {
                        targ -= *r;
                    }
                    '*' => {
                        let (d, r) = targ.div_rem(r);
                        assert_eq!(r, 0);
                        targ = d;
                    }
                    o => todo!("r {o:?}"),
                }
                println!("{} = {}", render((&num, &op), &l), targ);
                here = l;
            }
            other => todo!("{other:?}"),
        }
    }
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

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
