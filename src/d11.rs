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

    let all_tests = monk.iter().map(|m| m.2).product::<i64>();
    let mut inspections = vec![0usize; monk.len()];
    let mut curs: Vec<Vec<i64>> = monk.iter().map(|(s, _, _, _, _)| s.clone()).collect_vec();
    for round in 0..10000 {
        for mn in 0..curs.len() {
            let (_s, op, test, tru, fals) = &monk[mn];
            let this_monk = curs[mn].clone();
            curs[mn].clear();
            for mut item in this_monk {
                inspections[mn] += 1;
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

                item %= all_tests;

                let next = *if item % test == 0 { tru } else { fals };
                println!("round: {round} monkey: {mn} item new value: {item} thrown to: {next}");
                curs[next].push(item);
            }
        }
        for (mo, holding) in curs.iter().enumerate() {
            println!("{round} | {mo} | {holding:?}");
        }
    }
    println!("{:#?}", inspections);
    println!(
        "{:#?}",
        inspections
            .into_iter()
            .sorted()
            .rev()
            .take(2)
            .product::<usize>()
    );
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
