use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

pub fn solve() {
    let mut grid = include_str!("d13.txt")
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(p)
        .collect_vec();

    let d1 = p("[[2]]");
    let d2 = p("[[6]]");
    grid.push(d1.clone());
    grid.push(d2.clone());
    grid.sort();

    for line in &grid {
        println!("{line}");
    }

    let p1 = 1 + grid.iter().position(|x| x == &d1).unwrap();
    let p2 = 1 + grid.iter().position(|x| x == &d2).unwrap();

    println!("{p1} {p2} {}", p1 * p2);
}

#[derive(Debug, Clone, Eq)]
enum V {
    L(Vec<V>),
    A(i64),
}

impl Display for V {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            V::A(x) => write!(f, "{}", x),
            V::L(l) => write!(f, "[{}]", l.iter().map(|x| format!("{x}")).join(",")),
        }
    }
}

impl Ord for V {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use V::*;
        match (self, other) {
            (A(l), A(r)) => l.partial_cmp(r),
            (L(l), L(r)) => {
                for i in 0..l.len().min(r.len()) {
                    let l = &l[i];
                    let r = &r[i];
                    if l == r {
                        continue;
                    }
                    return l.partial_cmp(r);
                }
                l.len().partial_cmp(&r.len())
            }
            (l @ L(_), r @ A(_)) => l.partial_cmp(&L(vec![r.clone()])),
            (l @ A(_), r @ L(_)) => L(vec![l.clone()]).partial_cmp(r),
        }
    }
}

fn p(s: &str) -> V {
    let v: Value = serde_json::from_str(s).unwrap();
    u(v)
}

fn u(v: Value) -> V {
    match v {
        Value::Array(list) => V::L(list.into_iter().map(u).collect()),
        Value::Number(a) => V::A(a.as_i64().unwrap()),
        other => unreachable!("{other:?}"),
    }
}

#[test]
fn one() {
    assert!(p("[1,1,3,1,1]") < p("[1,1,5,1,1]"));
    assert!(p("[9]") > p("[[8,7,6]]"));
    assert!(p("[7,7,7,7]") > p("[7,7,7]"));
}
