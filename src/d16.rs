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
    derived.iter_mut().for_each(|x| x.1.sort_by_key(|x| x.1));
    println!("{derived:#?}");

    let best = search(
        &graph,
        &derived,
        [Key::new("AA"), Key::new("AA")],
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
    weights: &HashMap<Key, (usize, Box<[Key]>)>,
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
            .iter()
            .map(|(v, m)| weights[v].0 * (26 - m.0 as usize))
            .sum();
        // if fin == 1570 {
        //     let dbg = on.iter().map(|(v, m)| ((*m as usize), v, weights[v].0)).sorted().collect_vec();
        //     println!("{here} {dbg:?} {fin}");
        // }
        return fin;
    }

    let mut opts = Vec::with_capacity(12);

    let me_moving = min_me < min_ele;

    let here = if me_moving { me } else { ele };
    let minute = if me_moving { min_me } else { min_ele };

    // if we've already been here, then it's a waste of time
    if here != Key::new("AA") && on.insert(here, (minute, me_moving)).is_some() {
        let fin = on
            .iter()
            .map(|(v, m)| weights[v].0 * (26 - m.0 as usize))
            .sum();
        if fin == 1704 {
            let dbg = on
                .iter()
                .map(|(v, m)| ((m.0 as usize), v, m.1))
                .sorted()
                .collect_vec();
            println!("{here} {dbg:?} {fin}");
        }
        return fin;
    }

    for (dk, dmin) in &grid[&here] {
        if here == Key::new("AA") && on.is_empty() {
            println!("{me_moving} {dk} {dmin}");
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
        // memo.insert((here, minute, flatten(&on)), score);
        // println!("{minute} {here} -> {neigh} = {score}");
        opts.push(score);
    }

    *opts.iter().max().unwrap_or(&0)
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
