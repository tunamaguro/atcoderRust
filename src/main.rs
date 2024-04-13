use std::{cmp::Ordering, collections::BTreeMap};

use proconio::{input, marker::Chars};

trait Bound<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: PartialOrd> Bound<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let (mut low, mut high) = (0, self.len());
        while low + 1 < high {
            let mid = (low + high) / 2;
            if self[mid] < *x {
                low = mid;
            } else {
                high = mid;
            }
        }
        if self[low] < *x {
            low + 1
        } else {
            low
        }
    }

    fn upper_bound(&self, x: &T) -> usize {
        let (mut low, mut high) = (0, self.len());
        while low + 1 < high {
            let mid = (low + high) / 2;
            if self[mid] <= *x {
                low = mid;
            } else {
                high = mid;
            }
        }
        if self[low] <= *x {
            low + 1
        } else {
            low
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct TakahashiState {
    energy: usize,
    position: (usize, usize),
}

impl PartialOrd for TakahashiState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.energy.partial_cmp(&other.energy)
    }
}

impl Ord for TakahashiState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.energy.cmp(&other.energy)
    }
}

fn main() {
    input! {s:Chars,t:Chars}
    let mut t = t
        .iter()
        .map(|c| c.to_ascii_lowercase())
        .rev()
        .collect::<Vec<_>>();
    for si in s {
        if t.is_empty() {
            break;
        }
        let si = si.to_ascii_lowercase();
        let ti = t.last().unwrap();
        if si == *ti {
            t.pop();
        } else {
            continue;
        }
    }
    let mut ans = false;
    if t.len() == 1 && t.last() == Some(&'x') {
        ans = true;
    } else if t.is_empty() {
        ans = true
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
