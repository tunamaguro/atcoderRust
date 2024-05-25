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
    let mut min_bingo_time = usize::MAX;
    // 横列のビンゴがあったか判定する
    'outer: for row in 0..n {
        let mut min_t = 0;
        for col in 0..n {
            let pos = n * row + col;
            if let Some(c) = memo.get(&pos) {
                min_t = min_t.max(*c);
            } else {
                continue 'outer;
            }
        }
        min_bingo_time = min_bingo_time.min(min_t);
    }

    // 縦列のビンゴがあったか判定する
    'outer2: for col in 0..n {
        let mut min_t = 0;
        for row in 0..n {
            let pos = n * row + col;
            if let Some(c) = memo.get(&pos) {
                min_t = min_t.max(*c);
            } else {
                continue 'outer2;
            }
        }
        min_bingo_time = min_bingo_time.min(min_t);
    }

    // 斜めのビンゴがあったか判定する
    let mut rd_min_t = 0;
    let mut ld_min_t = 0;
    for i in 0..n {
        let rd_pos = n * i + i;
        let ld_pos = n * i + n - i - 1;
        if let Some(c) = memo.get(&rd_pos) {
            rd_min_t = rd_min_t.max(*c);
        }
        if let Some(c) = memo.get(&ld_pos) {
            ld_min_t = ld_min_t.max(*c);
        }
    }
    if rd_min_t!=0{
        min_bingo_time=min_bingo_time.min(rd_min_t);
    }
    if ld_min_t!=0{
        min_bingo_time=min_bingo_time.min(ld_min_t);
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
