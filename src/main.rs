#![allow(dead_code)]

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
    input! {_n:usize,m:usize,colors:[(i32,i32,char);m]}

    let colors = colors.into_iter().sorted().collect::<Vec<_>>();

    let mut min_y = i32::MAX;
    for (_x, y, c) in colors {
        if c == 'W' {
            min_y = min_y.min(y);
        } else if y >= min_y {
            println!("No");
            return;
        }
    }
    println!("Yes")
}
