use itertools::Itertools;

pub fn solve() {
    let mut dat = include_str!("d20.txt")
        .lines()
        .map(p)
        .enumerate()
        .collect_vec();

    let w = dat.len() as i64;

    for i in 0..dat.len() {
        let (start, (_orig, val)) = dat.iter().find_position(|(x, _)| *x == i).unwrap();
        let dest = usize::try_from((start as i64 + *val + (w - 1) + (w - 1)) % (w - 1)).unwrap();
        let taken = dat.remove(start);
        // println!("removed: {:?}",
        // dat.iter().map(|(_, x)| *x).collect_vec()
        // );
        dat.insert(if dest >= start { dest } else { dest }, taken);

        // println!(
        //     "{taken:?} moves from {start} to {dest}, value: {:?}",
        //     dat.iter().map(|(_, x)| *x).collect_vec()
        // )
    }

    let key = dat.iter().position(|(_, x)| 0 == *x).unwrap();
    for i in [1000, 2000, 3000] {
        println!("{}", dat[(key + i) % dat.len()].1);
    }

    // println!("{dat:?}")
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
