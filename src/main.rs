use proconio::{input, marker::Chars};

fn main() {
    input! {mut s:Chars}
    let n = s.len() / 2;
    for i in 0..n {
        s.swap(2 * i + 1, 2 * i);
    }
    println!(
        "{}",
        s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("")
    )
}
