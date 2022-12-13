use aoc_lib::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering::*;

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(untagged)]
enum ListItem {
    L(Vec<ListItem>),
    V(i32),
}
use ListItem::*;

fn main() -> NullResult {
    let args = args(BIN)?;
    let now = Instant::now();

    let solution: usize = if !args.second {
        args.input
            .split("\n\n")
            .map(|pair_str| {
                let (left, right) = pair_str.split_once('\n').unwrap();
                let left: ListItem = serde_json::from_str(left).unwrap();
                let right: ListItem = serde_json::from_str(right).unwrap();
                None.or(check_pair(&left, &right))
            })
            .enumerate()
            .filter_map(|(i, v)| match v {
                Some(true) => Some(i + 1),
                _ => None,
            })
            .sum()
    } else {
        let div1: ListItem = serde_json::from_str("[[2]]").unwrap();
        let div2: ListItem = serde_json::from_str("[[6]]").unwrap();
        let mut lines: Vec<_> = args
            .input
            .lines()
            .filter(|&l| l != "")
            .map(|l| serde_json::from_str::<ListItem>(l).unwrap())
            .collect();
        lines.push(div1.clone());
        lines.push(div2.clone());
        lines.sort_by(|left, right| match check_pair(left, right) {
            Some(true) => Less,
            None => Equal,
            Some(false) => Greater,
        });
        lines
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if v == &div1 || v == &div2 {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .product()
    };

    result(solution, now.elapsed(), &args)
}

fn check_pair(left: &ListItem, right: &ListItem) -> Option<bool> {
    match (left, right) {
        (V(l), V(r)) => match l.cmp(r) {
            Less => Some(true),
            Equal => None,
            Greater => Some(false),
        },
        (L(l), L(r)) => {
            if l.len() == 0 && r.len() > 0 {
                return Some(true);
            }
            if r.len() == 0 && l.len() > 0 {
                return Some(false);
            }

            let len = l.len().min(r.len());
            let it_win = (0..len)
                .fold(None, |acc, idx| acc.or(check_pair(&l[idx], &r[idx])));
            it_win.or_else(|| match l.len().cmp(&r.len()) {
                Less => Some(true),
                Equal => None,
                Greater => Some(false),
            })
        }
        (ll @ L(_), v @ V(_)) => check_pair(ll, &L(vec![v.clone()])),
        (v @ V(_), rr @ L(_)) => check_pair(&L(vec![v.clone()]), rr),
    }
}
