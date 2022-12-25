use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::{HashSet, VecDeque};

pub fn solve() {
    let nums = include_str!("d25.txt").lines().map(p).collect_vec();
    let target = nums.iter().sum::<i64>();

    let mut max = 0i64;
    let mut i = 0;
    loop {
        max += 2 * 5i64.pow(i);
        i += 1;
        if max > target {
            break;
        }
        println!("{max} {} {}", max > target, i);
    }

    let mut rng = rand::thread_rng();
    loop {
        let v = [2i64, 1, 0, -1, -2]
            .choose_multiple(&mut rng, i as usize)
            .copied()
            .collect_vec();
        let sum = v
            .iter()
            .enumerate()
            .map(|(p, v)| 5i64.pow(p as u32) * v)
            .sum::<i64>();
        if sum == target {
            println!("{sum} {v:?}");
            break;
        }
    }

    println!("{target:?}")
}

fn p(s: &str) -> i64 {
    s.chars()
        .map(|c| match c {
            '2' => 2i64,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            o => unreachable!("{o:?}"),
        })
        .enumerate()
        .map(|(p, v)| 5i64.pow((s.len() - p - 1) as u32) * v)
        .sum::<i64>()
}

#[test]
fn one() {
    assert_eq!(p("1"), 1);
}
#[test]
fn two() {
    assert_eq!(p("2"), 2);
}
#[test]
fn three() {
    assert_eq!(p("1="), 3);
}
#[test]
fn four() {
    assert_eq!(p("1-"), 4);
}
#[test]
fn five() {
    assert_eq!(p("10"), 5);
}
#[test]
fn ten() {
    assert_eq!(p("20"), 10);
}
