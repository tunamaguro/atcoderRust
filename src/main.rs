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
            dbg!(format!("row = {}, col = {}, pos = {}", row, col, pos));
            if let Some(c) = memo.get(&pos) {
                min_t = min_t.max(*c);
            } else {
                dbg!(format!(
                    "row = {}, col = {}, pos = {} was skip",
                    row, col, pos
                ));
                continue 'outer;
            }
        }
        dbg!(row, min_t);
        min_bingo_time = min_bingo_time.min(min_t);
    }

    // 縦列のビンゴがあったか判定する
    'outer2: for col in 0..n {
        let mut min_t = 0;
        for row in 0..n {
            let pos = n * row + col;
            dbg!(format!("row = {}, col = {}, pos = {}", row, col, pos));
            if let Some(c) = memo.get(&pos) {
                min_t = min_t.max(*c);
            } else {
                dbg!(format!(
                    "row = {}, col = {}, pos = {} was skip",
                    row, col, pos
                ));
                continue 'outer2;
            }
        }
        dbg!(col, min_t);
        min_bingo_time = min_bingo_time.min(min_t);
    }

    // 斜めのビンゴがあったか判定する
    let mut rd_min_t = vec![];
    let mut ld_min_t = vec![];
    for i in 0..n {
        let rd_pos = n * i + i;
        let ld_pos = n * i + n - i - 1;
        dbg!(format!(
            "i = {}, rd_pos = {}, ld_pos = {}",
            i, rd_pos, ld_pos
        ));
        if let Some(c) = memo.get(&rd_pos) {
            rd_min_t.push(c)
        }
        if let Some(c) = memo.get(&ld_pos) {
            ld_min_t.push(c);
        }
    }
    if rd_min_t.len() == n {
        // dbg!(min_bingo_time, rd_min_t);
        min_bingo_time = min_bingo_time.min(**rd_min_t.iter().max().unwrap());
    }
    if ld_min_t.len() == n {
        // dbg!(min_bingo_time, ld_min_t);
        min_bingo_time = min_bingo_time.min(**ld_min_t.iter().max().unwrap());
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
