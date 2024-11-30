#![allow(dead_code)]
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

fn main() {
    input! {_n:usize,d:usize,s:Chars}
    let mut should_eat = d;
    let mut ans = vec![];
    for si in s.iter().rev() {
        if *si == '@' && should_eat > 0 {
            ans.push('.');
            should_eat -= 1;
        } else {
            ans.push(*si);
        }
    }

    println!("{}", ans.iter().rev().collect::<String>())
}
