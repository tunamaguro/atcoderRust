use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {h:usize,_w:usize,s:[Chars;h]}
    let mut ans = s.clone();

    for (i, r) in s.iter().enumerate() {
        let mut has_before = false;
        for (j, c) in r.iter().enumerate() {
            if has_before && c == &'T' {
                ans[i][j - 1] = 'P';
                ans[i][j] = 'C';
                has_before = false;
                continue;
            }
            if c == &'T' {
                has_before = true;
                continue;
            }
            has_before = false
        }
    }
    let ans = ans
        .iter()
        .map(|r| r.iter().map(|x| x.to_string()).join(""))
        .collect::<Vec<_>>();
    for r in ans {
        println!("{}", r)
    }
}
