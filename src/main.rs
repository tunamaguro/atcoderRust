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
    input! {k:usize,s:Chars,t:Chars}
    if s.len() == t.len() {
        let mut diffs = 0;
        for (si, ti) in s.iter().zip(t.iter()) {
            if si != ti {
                diffs += 1;
            }
        }

        if diffs <= k {
            println!("Yes");
        } else {
            println!("No")
        }
    } else {
        let mut sidx = 0;
        let mut tidx = 0;
        let s_is_longer = s.len() > t.len();

        let mut diffs = 0;
        while s.len() > sidx && t.len() > tidx {
            if s[sidx] != t[tidx] {
                diffs += 1;
                if s_is_longer {
                    sidx += 1;
                } else {
                    tidx += 1
                }
            }

            sidx += 1;
            tidx += 1;
        }

        let len_diff = s.len().max(t.len()) - s.len().min(t.len());

        if diffs <= k && len_diff <= k {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
