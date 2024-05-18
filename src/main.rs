use std::vec;

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
    input! {n:usize,mut cards:[(usize,usize);n]}
    let mut cards = cards.into_iter().enumerate().collect_vec();
    cards.sort_by(|a, b| a.1 .0.cmp(&b.1 .0).reverse().then(a.1 .1.cmp(&b.1 .1)));
    // println!("{:?}", cards);
    let mut ans = vec![];
    ans.push(cards[0]);
    for &(idx, (tuyosa, cost)) in cards.iter().skip(1) {
        let l = ans.last().unwrap();
        // println!("{:?} tuyosa:{}, cost:{}", l, tuyosa, cost);
        if l.1 .0 > tuyosa && l.1 .1 < cost {
            continue;
        } else {
            ans.push((idx, (tuyosa, cost)));
        }
    }
    let ans_str = ans
        .iter()
        .map(|x| (x.0 + 1))
        .sorted()
        .map(|x| x.to_string())
        .join(" ");
    println!("{}", ans.len());
    println!("{}", ans_str);
}
