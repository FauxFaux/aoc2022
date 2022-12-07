use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let mut lines = include_str!("d7.txt").lines().peekable();
    let mut here: Vec<&str> = Vec::new();

    let mut cont = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let cmd = line.strip_prefix("$ ").expect("command");

        println!("{cmd:?} in {here:?}");

        match (cmd, cmd.split_once(' ')) {
            (_, Some(("cd", loc))) => match loc {
                "/" => here.clear(),
                ".." => {
                    here.pop().expect("downwards");
                }
                x if x.chars().all(|c: char| c == '.' || c.is_ascii_alphabetic()) => here.push(x),
                other => panic!("cd to {other:?}"),
            },
            ("ls", _) => {
                let mut stuff = Vec::new();

                while lines
                    .peek()
                    .map(|l| !l.starts_with("$ "))
                    .unwrap_or_default()
                {
                    match lines.next().expect("peeked").split_once(' ') {
                        Some(("dir", name)) => (),
                        Some((size, name)) => {
                            stuff.push(p(size));
                        }
                        other => unreachable!("{other:?}"),
                    }
                }
                cont.insert(here.clone(), stuff);
            }
            other => panic!("{other:?}"),
        }
    }

    println!("{cont:#?}");

    let to_free = 30000000
        - (70000000
            - cont
                .iter()
                .map(|(_, v)| v.iter().sum::<usize>())
                .sum::<usize>());

    let mut sizes = Vec::new();

    for (k, v) in cont.iter() {
        let mut size = 0;
        for (s, sv) in cont.iter() {
            if s.starts_with(&k) {
                size += sv.iter().sum::<usize>();
            }
        }

        sizes.push((k.clone(), size));
    }

    sizes.sort_by_key(|(_, v)| *v);
    println!("{:?}", sizes.iter().find(|(_, v)| *v >= to_free).unwrap());
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
