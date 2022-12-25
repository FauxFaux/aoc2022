use itertools::Itertools;
use rand::prelude::IteratorRandom;
use std::time::Instant;

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
    let len = i as usize;

    let start = Instant::now();
    let mut guess = vec![1i64; len];
    loop {
        let sum: i64 = guess
            .iter()
            .enumerate()
            .map(|(p, v)| 5i64.pow(p as u32) * *v)
            .sum();
        let diff = target - sum;
        if diff % (1024 * 1024) == 0 {
            println!("{diff} {guess:?}");
        }
        if diff == 0 {
            break;
        }
        // (-2i8..=3)
        let digit = diff.abs().ilog(5) as i8 + (-2i8..=2).choose(&mut rng).unwrap();
        if digit >= len as i8 || digit < 0 {
            continue;
        }
        let digit = digit as usize;
        guess[digit] = if diff > 0 {
            if guess[digit] != 2 {
                guess[digit] + 1
            } else {
                guess[digit]
            }
        } else {
            if guess[digit] != -2 {
                guess[digit] - 1
            } else {
                guess[digit]
            }
        }
    }

    println!("{:?}", start.elapsed());

    let chars = b"=-012";

    println!(
        "{target:?} = {}",
        guess
            .iter()
            .rev()
            .map(|c| chars[(c + 2) as usize] as char)
            .collect::<String>()
    );
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
