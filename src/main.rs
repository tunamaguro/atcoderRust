use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {_n:usize,s:Chars}
    let mut memo = HashSet::new();
    let mut point = (0, 0);
    memo.insert(point);
    for c in s {
        match c {
            'L' => {
                point = (point.0 - 1, point.1);
            }
            'R' => {
                point = (point.0 + 1, point.1);
            }
            'U' => {
                point = (point.0, point.1 + 1);
            }
            'D' => {
                point = (point.0, point.1 - 1);
            }
            _ => unreachable!(),
        }
        if memo.contains(&point) {
            println!("Yes");
            return;
        }
        memo.insert(point);
    }
    println!("No")
}
