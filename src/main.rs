use proconio::{input, marker::Chars};

fn main() {
    input! {h:usize,w:usize, s:[Chars;h]}
    let p = '#';

    let mut ans = true;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != p {
                continue;
            }
            let mut con = false;
            if i > 0 {
                con |= s[i - 1][j] == p;
            }
            if i < (h - 1) {
                con |= s[i + 1][j] == p
            }
            if j > 0 {
                con |= s[i][j - 1] == p;
            }
            if j < (w - 1) {
                con |= s[i][j + 1] == p
            }
            ans &= con;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
