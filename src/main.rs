use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let a: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .into_iter()
        .collect_vec();
    for (i, c) in s.iter().enumerate() {
        if a.contains(c) {
            println!("{}", i + 1);
            return;
        }
    }
}
