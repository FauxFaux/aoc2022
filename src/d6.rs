use itertools::Itertools;
use maplit::hashset;

pub fn solve() {
    let pos = include_str!("d6.txt")
        .chars()
        .tuple_windows()
        .position(|(a, b, c, d)| hashset! {a,b,c,d}.len() == 4)
        .unwrap()
        + 4;
    println!("{pos:?}")
}
