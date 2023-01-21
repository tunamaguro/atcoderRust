use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let mut ans = 0;
    let mut i: usize = 0;
    while s.len() > i {
        if s.len() > i + 1 && s[i] == '0' && s[i + 1] == '0' {
            i += 1;
        }
        ans += 1;

        i += 1;
    }
    println!("{}", ans)
}
