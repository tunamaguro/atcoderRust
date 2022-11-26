use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let mut ans = 0;
    for c in s {
        if c == 'v' {
            ans += 1;
        } else if c == 'w' {
            ans += 2;
        }
    }
    println!("{}", ans)
}
