use std::collections::BTreeMap;

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
    input! {n:usize,h:[i32;n]}
    let mut cost = vec![i32::MAX; n];
    cost[0] = 0;
    for i in 0..n {
        let cur_cost = cost[i];
        for j in (i + 1)..n.min(i + 3) {
            let c = (h[i] - h[j]).abs();
            cost[j] = cost[j].min(c + cur_cost)
        }
    }
    println!("{}", cost.last().unwrap())
}
