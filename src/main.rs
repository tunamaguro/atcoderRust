use std::collections::BTreeMap;

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
    input! {n:usize,t:usize,a:[usize;t]}

    let mut memo = BTreeMap::new();
    for (ti, ai) in a.iter().enumerate() {
        memo.insert(ai - 1, ti + 1);
    }

    let mut min_bingo_time = 10000000;
    // 縦列のビンゴがあったか判定する
    for row in 0..n {
        let mut called_time = vec![];
        for col in 0..n {
            let pos = n * row + col;
            if let Some(c) = memo.get(&pos) {
                called_time.push(c)
            }
        }
        if called_time.len() == n {
            let min_t = called_time.iter().max().unwrap();
            min_bingo_time = min_bingo_time.min(**min_t);
        }
    }
    // 横列のビンゴがあったか判定する
    for col in 0..n {
        let mut called_time = vec![];
        for row in 0..n {
            let pos = n * row + col;
            if let Some(c) = memo.get(&pos) {
                called_time.push(c)
            }
        }
        if called_time.len() == n {
            let min_t = called_time.iter().max().unwrap();
            min_bingo_time = min_bingo_time.min(**min_t);
        }
    }
    // 斜めのビンゴがあったか判定する
    let mut rd_called_time = vec![]; // 左上->右下
    let mut ld_called_time = vec![]; // 右上->左下
    for i in 0..n {
        let rd_pos = n * i + i;
        let ld_pos = n * i + n - i;
        if let Some(c) = memo.get(&rd_pos) {
            rd_called_time.push(c)
        }
        if let Some(c) = memo.get(&ld_pos) {
            ld_called_time.push(c)
        }
    }
    if rd_called_time.len() == n {
        let min_t = rd_called_time.iter().max().unwrap();
        min_bingo_time = min_bingo_time.min(**min_t);
    }
    if ld_called_time.len() == n {
        let min_t = ld_called_time.iter().max().unwrap();
        min_bingo_time = min_bingo_time.min(**min_t);
    }

    println!(
        "{}",
        if min_bingo_time < 10000000 {
            format!("{}", min_bingo_time)
        } else {
            "-1".to_string()
        }
    )
}
