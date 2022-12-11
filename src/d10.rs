use itertools::Itertools;

pub fn solve() {
    let cmds = include_str!("d10.txt")
        .lines()
        .map(|l| l.split_once(' ').map(|(_, c)| p(c)))
        .collect_vec();

    println!("{cmds:?}");

    let mut pixels = [false; 240];

    let mut check = |pc, x| {
        let off = (pc as i64 - 1) % 40;
        if x >= off - 1 && x <= off + 1 {
            pixels[pc - 1] = true;
        }
    };

    let mut x = 1i64;
    let mut pc = 0usize;

    for cmd in cmds {
        match cmd {
            None => {
                pc += 1;
                check(pc, x);
            }
            Some(v) => {
                pc += 1;
                check(pc, x);
                pc += 1;
                check(pc, x);
                x += v;
            }
        }
    }

    let grid = pixels
        .iter()
        .map(|&x| if x { '#' } else { '.' })
        .collect::<String>();
    let splut = regex::Regex::new("(.{40})")
        .unwrap()
        .replace_all(&grid, "$1\n");
    println!("{splut}");
}

fn p(s: &str) -> i64 {
    s.parse().unwrap()
}
