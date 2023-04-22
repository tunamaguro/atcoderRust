use proconio::{input, marker::Chars};

fn main() {
    input! {_n:usize,s:Chars}
    let mut ans = false;
    let mut find = false;
    for c in s {
        if c == '|' {
            find = !find
        }
        if find && c == '*' {
            ans = true
        }
    }

    println!("{}", if ans { "in" } else { "out" })
}
