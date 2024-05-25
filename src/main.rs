use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

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

fn main() {
    input! {n:usize,m:usize,mut a:[i32;n],mut b:[i32;m]}
    let mut memo = HashSet::new();
    for &ai in a.iter() {
        memo.insert(ai);
    }
    let mut ans = false;
    let c = a.into_iter().chain(b.into_iter()).sorted().collect_vec();
    for (c1, c2) in c.iter().zip(c.iter().skip(1)) {
        if memo.contains(c1) && memo.contains(c2) {
            ans = true
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
