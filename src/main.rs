use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {n:usize,s:[Chars;n]}
    let mut memo = HashSet::new();
    let mark = "HDCS".chars().into_iter().collect::<Vec<_>>();
    let num = "A23456789TJQK".chars().into_iter().collect::<Vec<_>>();
    let mut ans = true;
    for si in s {
        if mark.contains(&si[0]) && num.contains(&si[1]) {
            if memo.contains(&si) {
                ans = false
            }
            memo.insert(si);
        } else {
            ans = false
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
