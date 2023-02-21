use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {n:usize,s:Chars}
    let mut ans = 0;
    for i in 0..=n {
        let x = &s[0..i];
        let y = &s[(i)..n];
        let k = x.iter().collect::<HashSet<_>>();
        let l = y.iter().collect::<HashSet<_>>();

        let mut c = 0;
        for p in l {
            if k.contains(&p) {
                c += 1;
            };
        }
        ans = std::cmp::max(c, ans);
    }
    println!("{}", ans)
}
