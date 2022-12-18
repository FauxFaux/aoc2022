use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Key(u8);

pub fn solve() {
    let mut key_names = Vec::new();
    key_names.push("AA");
    let mut name = |s: &'static str| match key_names.iter().position(|x| *x == s) {
        Some(x) => Key(x as u8),
        None => {
            let x = key_names.len();
            key_names.push(s);
            Key(x as u8)
        }
    };

    let graph = include_str!("d16.txt")
        .lines()
        .map(|x| {
            let x = x
                .split(|c| matches!(c, ';' | '=' | ' ' | ','))
                .collect_vec();
            let v = name(x[1]);
            let r = p(x[5]);
            let t = x[11..]
                .iter()
                .filter(|x| !x.is_empty())
                // stupid ass-language
                .map(|x| name(x))
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

    interesting.insert(0, Key(0));

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
    derived.iter_mut().for_each(|x| x.1.sort_by_key(|x| x.1));
    println!("{derived:#?}");

    let mut weights = [0u8; 64];
    for (k, v) in &graph {
        weights[k.0 as usize] = v.0 as u8;
    }

    let best = search(
        &weights,
        &derived,
        [Key(0), Key(0)],
        [0, 0],
        HashMap::with_capacity(8),
    );

    println!("{best:?}")
}

fn find(grid: &HashMap<Key, (usize, Box<[Key]>)>, from: Key, to: Key, dist: u8) -> u8 {
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

fn search(
    weights: &[u8; 64],
    grid: &HashMap<&Key, Vec<(&Key, u8)>>,
    // memo: &mut HashMap<(Key, u8, Vec<(Key, u8)>), usize>,
    here: [Key; 2],
    minute: [u8; 2],
    mut on: HashMap<Key, (u8, bool)>,
) -> usize {
    // if let Some(existing) = memo.get(&(here, minute, flatten(&on))) {
    //     return *existing;
    // }

    let [me, ele] = here;
    let [min_me, min_ele] = minute;

    if min_me >= 26 && min_ele >= 26 {
        let fin = on
            .into_iter()
            .map(|(v, m)| weights[v.0 as usize] as usize * (26 - m.0 as usize))
            .sum();
        // if fin == 1570 {
        //     let dbg = on.iter().map(|(v, m)| ((*m as usize), v, weights[v].0)).sorted().collect_vec();
        //     println!("{here} {dbg:?} {fin}");
        // }
        return fin;
    }

    let mut opts = Vec::with_capacity(12);

    for me_moving in [true, false] {
        let here = if me_moving { me } else { ele };
        let minute = if me_moving { min_me } else { min_ele };

        if minute >= 26 {
            continue;
        }

        // if we've already been here, then it's a waste of time
        if here != Key(0) && on.insert(here, (minute, me_moving)).is_some() {
            continue;
            let fin = on
                .iter()
                .map(|(v, m)| {
                    weights[v.0 as usize] as usize * (26usize.saturating_sub(m.0 as usize))
                })
                .sum();
            if fin >= 1855 {
                let dbg = on
                    .iter()
                    .map(|(v, m)| ((m.0 as usize), v, m.1))
                    .sorted()
                    .collect_vec();
                println!("{here:?} {dbg:?} {fin}");
            }
            opts.push(fin);
            continue;
        }

        for (dk, dmin) in &grid[&here] {
            if here == Key(0) && on.is_empty() {
                println!("{me_moving} {dk:?} {dmin}");
            }
            if **dk == Key(0) || on.contains_key(dk) {
                continue;
            }
            let score = search(
                weights,
                grid,
                if me_moving { [**dk, ele] } else { [me, **dk] },
                if me_moving {
                    [min_me + 1 + dmin, min_ele]
                } else {
                    [min_me, min_ele + 1 + dmin]
                },
                on.clone(),
            );
            if here == Key(0) && on.is_empty() {
                println!("-> {score}");
            }
            // memo.insert((here, minute, flatten(&on)), score);
            // println!("{minute} {here} -> {neigh} = {score}");
            opts.push(score);
        }
    }

    *opts.iter().max().unwrap_or(&0)
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
