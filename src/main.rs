use proconio::{input, marker::Chars};

fn main() {
    input! {mut s:Chars}
    let mut find = false;
    s.reverse();
    let target = 'a';
    for (i, c) in s.iter().enumerate() {
        if c == &target {
            println!("{}", s.len() - i);
            find = true;
            break;
        }
    }
    if !find {
        println!("{}", -1)
    }
}
