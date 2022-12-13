use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

pub fn solve() {
    let grid = include_str!("d13.txt")
        .lines()
        .tuples()
        .map(|(a, b, _)| (p(a), p(b)))
        .collect_vec();

    let sorted = grid
        .into_iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(p, _)| p + 1)
        .sum::<usize>();
    println!("{:#?}", sorted);
}

#[derive(Debug, Clone, PartialEq)]
enum V {
    L(Vec<V>),
    A(i64),
}

impl PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
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
