use itertools::Itertools;

pub fn solve() {
    let lines = include_str!("d5.txt").lines().collect_vec();

    let (grid, moves) = lines.split_at(lines.iter().position(|x| x.is_empty()).unwrap());
    let mut grid = grid
        .into_iter()
        .map(|l| {
            format!("{l} ")
                .chars()
                .tuples()
                .map(|(_o, c, _c, _s)| c)
                .collect_vec()
        })
        .collect_vec();
    let _key = grid.pop().unwrap();
    let width = grid.iter().map(|x| x.len()).max().unwrap();
    let mut g = (0..width).map(|_| Vec::<char>::new()).collect_vec();
    for line in grid.iter().rev() {
        for (i, c) in line.into_iter().enumerate() {
            if c.is_ascii_whitespace() {
                continue;
            }
            g[i].push(*c);
        }
    }

    let moves = moves
        .into_iter()
        .skip(1) // split point
        .map(|line| {
            line.split_whitespace()
                .tuples()
                .map(|(_m, c, _f, s, _t, d)| (p(c), p(s) - 1, p(d) - 1))
                .next()
                .unwrap()
        })
        .collect_vec();

    for (count, source, dest) in moves.into_iter() {
        let src = &mut g[source];
        let rem = src.len() - count;
        let taken = src[rem..].to_vec();
        g[dest].extend(taken);
        g[source].truncate(rem);
    }

    println!("{g:?}");

    println!(
        "{}",
        g.into_iter().map(|x| x[x.len() - 1]).collect::<String>()
    );
}

fn p(s: &str) -> usize {
    s.parse().unwrap()
}
