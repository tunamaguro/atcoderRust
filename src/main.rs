use proconio::{input, marker::Chars};

fn main() {
    input! {n:usize,m:usize,s:[Chars;n]}
    let mut ans = 0;
    for a in 0..n {
        for b in a..n {
            if a == b {
                continue;
            }
            let mut c = 0;
            for j in 0..m {
                if s[a][j] == 'o' || s[b][j] == 'o' {
                    c += 1;
                }
            }
            if c == m {
                ans += 1
            }
        }
    }
    println!("{}", ans)
}
