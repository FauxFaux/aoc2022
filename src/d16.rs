use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Key(u16);

impl Key {
    fn new(s: &str) -> Key {
        let [a, b]: [u8; 2] = s.bytes().collect_vec().try_into().unwrap();
        Key((a as u16) * 256 + b as u16)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (self.0 / 256) as u8 as char,
            self.0 as u8 as char
        )
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn solve() {
    let graph = include_str!("d16.txt")
        .lines()
        .map(|x| {
            let x = x
                .split(|c| matches!(c, ';' | '=' | ' ' | ','))
                .collect_vec();
            let v = Key::new(x[1]);
            let r = p(x[5]);
            let t = x[11..]
                .iter()
                .filter(|x| !x.is_empty())
                // stupid ass-language
                .map(|x| Key::new(x))
                .collect_vec()
                .into_boxed_slice();
            (v, (r, t))
        })
        .collect::<HashMap<Key, (usize, _)>>();

    let mut interesting = graph
        .iter()
        .filter(|(_, (value, _))| *value != 0)
        .map(|(dest, _)| *dest)
        .collect_vec();

    interesting.insert(0, Key::new("AA"));

    let mut derived = HashMap::with_capacity(interesting.len());

    for from in &interesting {
        for to in &interesting {
            if from == to {
                continue;
            }
            let dist = find(&graph, *from, *to, 0);
            derived.entry(from).or_insert(Vec::new()).push((to, dist));
        }
    }
    println!("{derived:?}");

    let mut memo = HashMap::with_capacity(9000);
    let best = search(
        &graph,
        &mut memo,
        Key::new("AA"),
        0,
        HashMap::with_capacity(8),
    );

    println!("{best:?}")
}

fn find(grid: &HashMap<Key, (usize, Box<[Key]>)>, from: Key, to: Key, dist: usize) -> usize {
    if dist > 13 {
        return dist;
    }
    if from == to {
        return dist;
    }

    let mut guesses = Vec::with_capacity(8);
    for cand in grid[&from].1.iter() {
        guesses.push(find(grid, *cand, to, dist + 1));
    }

    guesses.into_iter().min().unwrap_or(30)
}

fn flatten(on: &HashMap<Key, u8>) -> Vec<(Key, u8)> {
    on.iter().map(|(x, y)| (*x, *y)).sorted().collect()
}

fn search(
    grid: &HashMap<Key, (usize, Box<[Key]>)>,
    memo: &mut HashMap<(Key, u8, Vec<(Key, u8)>), usize>,
    here: Key,
    minute: u8,
    on: HashMap<Key, u8>,
) -> usize {
    if let Some(existing) = memo.get(&(here, minute, flatten(&on))) {
        return *existing;
    }

    if minute == 30 {
        let fin = on.iter().map(|(v, m)| grid[v].0 * (29 - *m as usize)).sum();
        // if fin > 1651 {
        //     let dbg = on.iter().map(|(v, m)| (grid[v].0, (*m as usize))).collect_vec();
        //     println!("{here} {dbg:?} {fin}");
        // }
        return fin;
    }

    let mut opts = Vec::with_capacity(6);

    let (v, neigh) = &grid[&here];
    if *v != 0 && !on.contains_key(&here) {
        let mut on = on.clone();
        on.insert(here, minute);
        let key = flatten(&on);
        let score = search(grid, memo, here, minute + 1, on);
        memo.insert((here, minute, key), score);
        opts.push(score);
    }

    for neigh in neigh.iter() {
        let score = search(grid, memo, *neigh, minute + 1, on.clone());
        // memo.insert((here, minute, flatten(&on)), score);
        // println!("{minute} {here} -> {neigh} = {score}");
        opts.push(score);
    }

    *opts.iter().max().unwrap_or(&0)
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
