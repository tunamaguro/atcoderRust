use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {s:Chars}
    let mut memo = HashMap::new();
    let mut ans = true;
    for c in s {
        if memo.contains_key(&c) {
            ans = false
        }
        memo.entry(c).or_insert(true);
    }
    let alphabet = (b'a'..=b'z').map(|x| x as char).collect::<Vec<_>>();
    let balphabet = (b'A'..=b'Z').map(|x| x as char).collect::<Vec<_>>();
    let mut has_aa = false;
    for a in alphabet {
        for aa in &balphabet {
            has_aa = has_aa || (memo.contains_key(&a) && memo.contains_key(aa))
        }
    }
    ans = ans && has_aa;
    println!("{}", if ans { "Yes" } else { "No" })
}
