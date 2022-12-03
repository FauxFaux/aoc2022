use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() {
    let pairs = include_str!("d3.txt")
        .trim()
        .lines()
        .tuples()
        .map(|(l, r, b)| {
            let l: HashSet<char> = l.chars().collect();
            let r: HashSet<char> = r.chars().collect();
            let b: HashSet<char> = b.chars().collect();
            let d = l
                .intersection(&r)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&b)
                .copied()
                .collect_vec();
            assert_eq!(1, d.len());
            d[0] as u8
        })
        .map(|c| match c as char {
            'a'..='z' => c as i64 - 'a' as i64 + 1,
            'A'..='Z' => c as i64 - 'A' as i64 + 27,
            _ => unreachable!(),
        })
        .sum::<i64>();

    println!("{pairs:?}");
}
