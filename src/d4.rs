use itertools::Itertools;

pub fn solve() {
    let ranges = include_str!("d4.txt")
        .lines()
        .map(|l| l.split_once(',').expect("comma"))
        .map(|(a, b)| (parse_range(a), parse_range(b)))
        .collect_vec();
    // println!("{}", ranges.len());
    let count = ranges
        .into_iter()
        .filter(|(a, b)| (b.0..=b.1).into_iter().any(|b| (a.0..=a.1).contains(&b)))
        .count();
    println!("{count:#?}")
}
fn parse_range(s: &str) -> (i64, i64) {
    let (l, r) = s.split_once('-').expect("range");
    (l.parse().expect("l"), r.parse().expect("r"))
}
