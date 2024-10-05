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
    input! {n:usize,k:[i64;n]}
    let s_total: i64 = k.iter().sum();
    let s = s_total / 2;

    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut total = 0;
        for i in 0..n {
            if bit & (1 << i) > 0 {
                total += k[i];
            }
        }

        let a = (s - total).abs();
        let b = (s - ans).abs();
        // dbg!(ans, total, s, a, b);
        if a < b {
            ans = total;
        }
    }

    println!("{}", ans.max(s_total - ans))
}
