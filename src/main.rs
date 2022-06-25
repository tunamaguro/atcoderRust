use proconio::{input, marker::Chars};
use std::cmp::max;
fn main() {
    input! {n:usize,s:Chars,w:[usize;n]}
    let mut all_adult = 0;
    let mut all_child = 0;
    for c in &s {
        match c {
            '1' => all_adult += 1,
            '0' => all_child += 1,
            _ => unreachable!(),
        }
    }
    let mut m: Vec<_> = s.iter().zip(w).collect();
    m.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(b.0)
        }
    });
    let mut adult_count = 0;
    let mut child_count = 0;
    let mut ans = 0;
    let mut before: usize = 0;
    for (c, w) in &m {
        if before == *w {
            match c {
                '1' => adult_count += 1,
                '0' => child_count += 1,
                _ => unreachable!(),
            }
        } else {
            before = *w;
            let fx = all_adult - adult_count + child_count;
            ans = max(ans, fx);
            match c {
                '1' => adult_count += 1,
                '0' => child_count += 1,
                _ => unreachable!(),
            }
        }
    }
    println!("{}", max(ans, all_child));
}
