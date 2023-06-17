use proconio::input;
use std::cmp::max;

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
    input! {n:usize,menu:[(i64,i64);n]}
    // i番目まででのおいしさの総和の最大値
    // 2つ目の添え字はおなかを壊しているかいないか
    let mut dp = vec![vec![0; 2]; n + 1];
    for (i, &(x, y)) in menu.iter().enumerate() {
        let i = i + 1;
        let poison = x == 1;
        if poison {
            dp[i][1] = max(dp[i - 1][0] + y, dp[i - 1][1]);
            dp[i][0] = dp[i - 1][0]
        } else {
            let tmp = max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][0] = max(tmp + y, dp[i - 1][0]);
            dp[i][1] = dp[i - 1][1]
        }
    }
    println!("{}", max(dp[n][0], dp[n][1]))
}
