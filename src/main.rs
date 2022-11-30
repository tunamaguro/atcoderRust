use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {n:usize,a:[i64;n],q:usize}
    let mut memo = HashMap::<usize, i64>::new();
    let mut xq = -1;
    for _ in 0..q {
        input! {t:usize}
        match t {
            1 => {
                input! {b:i64}
                memo.clear();
                xq = b;
            }
            2 => {
                input! {i:usize,b:i64}
                if xq < 0 {
                    // xqで初期化される前
                    *memo.entry(i).or_insert(a[i - 1]) += b;
                } else {
                    *memo.entry(i).or_insert(xq) += b;
                }
                // println!("{:?}", memo)
            }
            3 => {
                input! {i:usize}
                let out = if xq < 0 {
                    // xqで初期化される前
                    memo.entry(i).or_insert(a[i - 1])
                } else {
                    memo.entry(i).or_insert(xq)
                };
                println!("{}", out)
                // println!("{:?}", memo)
            }
            _ => unreachable!(),
        }
    }
}
