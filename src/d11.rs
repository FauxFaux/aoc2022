use itertools::Itertools;

pub fn solve() {
    let monk = include_str!("d11.txt")
        .lines()
        .tuples()
        .map(|(_m, s, o, d, t, f, _)| {
            (
                sr(s, ": ").split(", ").map(p).collect_vec(),
                sr(o, " old ").split_once(' ').unwrap(),
                p(sr(d, " by ")),
                u(sr(t, " monkey ")),
                u(sr(f, " monkey ")),
            )
        })
        .collect_vec();

    let mut curs: Vec<Vec<i64>> = monk.iter().map(|(s, _, _, _, _)| s.clone()).collect_vec();
    for round in 0..20 {
        let mut new: Vec<Vec<i64>> = vec![Vec::new(); monk.len()];
        for (mn, cur) in curs.clone().into_iter().enumerate() {
            let (_s, op, test, tru, fals) = &monk[mn];
            for mut item in cur {
                let t = |s| match s {
                    "old" => item,
                    o => p(o),
                };
                match op {
                    ("+", v) => {
                        item += t(v);
                    }
                    ("*", v) => {
                        item *= t(v);
                    }
                    other => todo!("{other:?}"),
                }

                item /= 3;

                let next = *if item % test == 0 { tru } else { fals };
                println!("round: {round} monkey: {mn} item new value: {item} thrown to: {next}");
                new[next].push(item);
            }
        }
        curs = new;
        for (mo, holding) in curs.iter().enumerate() {
            println!("{round} | {mo} | {holding:?}");
        }
    }
}

fn sr<'s>(s: &'s str, needle: &str) -> &'s str {
    s.split_once(needle).unwrap().1
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
fn u(s: &str) -> usize {
    s.parse().unwrap()
}
