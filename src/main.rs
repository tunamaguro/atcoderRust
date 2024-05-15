use itertools::Itertools;
use proconio::input;
use std::{collections::BTreeMap, vec};

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
    input! {h:usize,w:usize,n:usize,mh:usize,mw:usize,mut grid:[[usize;w];h]}

    let mut count_set = vec![vec![vec![0; n]; w]; h];
    for hi in 0..h {
        for wi in 0..w {
            let v = grid[hi][wi] - 1;
            count_set[hi][wi][v] += 1;
            for ni in 0..n {
                if hi > 0 {
                    count_set[hi][wi][ni] += count_set[hi - 1][wi][ni];
                }
                if wi > 0 {
                    count_set[hi][wi][ni] += count_set[hi][wi - 1][ni];
                }
                if hi > 0 && wi > 0 {
                    count_set[hi][wi][ni] -= count_set[hi - 1][wi - 1][ni];
                }
            }
        }
    }

    for k in 0..=h - mh {
        for l in 0..=w - mw {
            let mut count = 0;
            // println!("k: {}, l: {}", k, l);
            let mh = mh - 1;
            let mw = mw - 1;
            for ni in 0..n {
                let mut num_ = count_set[h - 1][w - 1][ni] - count_set[k + mh][l + mw][ni];
                if k > 0 {
                    num_ += count_set[k - 1][l + mw][ni];
                }
                if l > 0 {
                    num_ += count_set[k + mh][l - 1][ni];
                }
                if k > 0 && l > 0 {
                    num_ -= count_set[k - 1][l - 1][ni];
                }
                if num_ > 0 {
                    count += 1;
                }
            }
            print!("{} ", count);
        }
        println!();
    }
}
