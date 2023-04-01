use proconio::{input, marker::Chars};

fn main() {
    input! {_n:usize,s:Chars}
    let mut ans = true;
    let mut c = s[0];
    for i in s.iter().skip(1) {
        if i == &c {
            ans = false
        }
        c = *i
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
