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

fn func(a: &[usize], c: usize, current_xor: usize, ans: &mut usize) {
    // c = 0(残り選択数が 0) ならば、最大値を更新して戻る
    if c == 0 {
        *ans = current_xor.max(*ans);
        return;
    }
    // idx が配列の長さに達したら探索を終了
    if a.is_empty() {
        return;
    }
    if let Some((first, rest)) = a.split_first() {
        // A[idx] を使う場合
        func(rest, c - 1, current_xor ^ first, ans);
        // A[idx] を使わない場合
        func(rest, c, current_xor, ans);
    }
}

fn main() {
    input! {n:usize,k:usize,a:[usize;n]}

    let mut ans = 0;

    if k <= n - k {
        func(&a, k, 0, &mut ans);
    } else {
        let mut all_xor = 0;
        for &val in &a {
            all_xor ^= val;
        }
        // XORは可逆性があるので、xorを取り直すと元の値に戻る
        // a xor b = c
        // c xor b = a
        func(&a, n - k, all_xor, &mut ans);
    }

    println!("{}", ans)
}
