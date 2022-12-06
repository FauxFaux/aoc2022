use itertools::Itertools;
use maplit::hashset;
use std::collections::HashSet;

pub fn solve() {
    let pos = include_str!("d6.txt")
        .chars()
        .collect_vec()
        .windows(14)
        .position(|x| x.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        + 14;
    println!("{pos:?}")
}
