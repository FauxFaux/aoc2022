use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let graph = include_str!("d16.txt")
        .lines()
        .map(|x| {
            let x = x
                .split(|c| matches!(c, ';' | '=' | ' ' | ','))
                .collect_vec();
            let v = x[1];
            let r = p(x[5]);
            let t = x[11..]
                .iter()
                .filter(|x| !x.is_empty())
                // stupid ass-language
                .map(|x| *x)
                .collect_vec()
                .into_boxed_slice();
            (v, (r, t))
        })
        .collect::<HashMap<&str, (usize, _)>>();

    let mut interesting = graph
        .iter()
        .filter(|(_, (value, _))| *value != 0)
        .map(|(dest, _)| *dest)
        .collect_vec();

    interesting.insert(0, "AA");
    for from in &interesting {
        for to in &interesting {
            if from == to {
                continue;
            }
            let dist = find(&graph, from, to, 0);
            println!("{from} {to} {dist}");
        }
    }

    let mut memo = HashMap::with_capacity(9000);
    let best = search(&graph, &mut memo, "AA", 0, HashMap::with_capacity(8));

    println!("{best:?}")
}

fn find(
    grid: &HashMap<&'static str, (usize, Box<[&'static str]>)>,
    from: &str,
    to: &str,
    dist: usize,
) -> usize {
    if dist > 13 {
        return dist;
    }
    if from == to {
        return dist;
    }

    let mut guesses = Vec::with_capacity(8);
    for cand in grid[from].1.iter() {
        guesses.push(find(grid, cand, to, dist + 1));
    }

    guesses.into_iter().min().unwrap_or(30)
}

fn flatten(on: &HashMap<&'static str, u8>) -> Vec<(&'static str, u8)> {
    on.iter().map(|(x, y)| (*x, *y)).sorted().collect()
}

fn search(
    grid: &HashMap<&'static str, (usize, Box<[&'static str]>)>,
    memo: &mut HashMap<(&'static str, u8, Vec<(&'static str, u8)>), usize>,
    here: &'static str,
    minute: u8,
    on: HashMap<&'static str, u8>,
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

    let (v, neigh) = &grid[here];
    if *v != 0 && !on.contains_key(here) {
        let mut on = on.clone();
        on.insert(here, minute);
        let key = flatten(&on);
        let score = search(grid, memo, here, minute + 1, on);
        memo.insert((here, minute, key), score);
        opts.push(score);
    }

    for neigh in neigh.iter() {
        let score = search(grid, memo, neigh, minute + 1, on.clone());
        // memo.insert((here, minute, flatten(&on)), score);
        // println!("{minute} {here} -> {neigh} = {score}");
        opts.push(score);
    }

    *opts.iter().max().unwrap_or(&0)
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
