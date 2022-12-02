use itertools::Itertools;
use std::io::BufRead;
use std::{fs, io};

pub fn solve() {
    let mut stacks = Vec::new();
    let mut stack: Vec<i64> = Vec::new();
    for line in io::BufReader::new(
        fs::File::open(std::env::args_os().nth(1).expect("usage: input")).expect("open"),
    )
    .lines()
    {
        let line = line.expect("line");
        let line = line.trim();

        if line.is_empty() {
            stacks.push(stack);
            stack = Vec::new();
            continue;
        }
        stack.push(line.parse().expect("parse"));
    }
    stacks.push(stack);

    println!(
        "{:?}",
        stacks
            .into_iter()
            .map(|x| x.into_iter().sum::<i64>())
            .sorted()
            .rev()
            .take(3)
            .sum::<i64>()
    );
}
