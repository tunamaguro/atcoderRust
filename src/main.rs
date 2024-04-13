use std::cmp::Ordering;

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
    input! {n:usize,a:[i32;n-1]}
    let total = -a.iter().sum::<i32>();
    println!("{}", total);
}
