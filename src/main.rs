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

    if (s.len() as i32 - t.len() as i32).abs() > k as i32 {
        println!("No");
        return;
    }

    let s_length = s.len();
    let t_length = t.len();

    let mut dp = vec![vec![usize::MAX; 2 * k + 1]; s_length + 1];

    dp[0][k] = 0;

    for i in 0..=s.len() {
        for dj in 0..=(2 * k) {
            let j = i as i32 + dj as i32 - k as i32;
            if j < 0 {
                continue;
            }
            let j = j as usize;
            if j > t_length {
                break;
            }

            if i > 0 && dj < 2 * k {
                dp[i][dj] = dp[i][dj].min(dp[i - 1][dj + 1] + 1)
            }
            if j > 0 && dj > 0 {
                dp[i][dj] = dp[i][dj].min(dp[i][dj - 1] + 1)
            }

            if i > 0 && j > 0 {
                let add = if s[i - 1] == t[j - 1] { 0 } else { 1 };
                dp[i][dj] = dp[i][dj].min(dp[i - 1][dj] + add)
            }
        }
    }

    if dp[s_length][k + t_length - s_length] <= k {
        println!("Yes")
    } else {
        println!("No")
    }
}
