use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars,t:Chars}
    let mut s_memo = BTreeMap::new();
    let mut t_memo = BTreeMap::new();
    for i in 0..s.len() {
        *s_memo.entry(s[i]).or_insert(0) += 1;
        *t_memo.entry(t[i]).or_insert(0) += 1;
    }
    let mut ans = true;
    let atcoder = "atcoder".chars().collect_vec();
    for (&k, &v) in &s_memo {
        if k == '@' {
            continue;
        }
        for _ in 0..v {
            let e = t_memo.entry(k).or_insert(0);
            if *e > 0 {
                *e -= 1;
                continue;
            }
            let joker = t_memo.entry('@').or_insert(0);
            if atcoder.contains(&k) && *joker > 0 {
                *joker -= 1;
                continue;
            }
            ans = false
        }
    }
    for (k, v) in t_memo {
        if v <= 0 {
            continue;
        }
        if !(atcoder.contains(&k) || k == '@') {
            ans = false;
            break;
        }
        let e = s_memo.entry('@').or_insert(0);
        for _ in 0..v {
            *e -= 1;
        }
        if *e < 0 {
            ans = false
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
