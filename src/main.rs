#![allow(dead_code)]
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
    input! {n:usize,m:usize,a:[i32;n],b:[i32;m]}
    let k = 2 * 100000 + 100;
    let mut id = vec![-1; k];
    let mut r = k;
    for (i, ai) in a.iter().enumerate() {
        while r as i32 > *ai {
            r -= 1;
            id[r] = i as i32 + 1;
        }
    }
    for bi in b {
        let bi = bi as usize;
        println!("{}", id[bi])
    }
}
