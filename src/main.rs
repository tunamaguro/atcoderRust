use proconio::{input, marker::Chars};

fn main() {
    input! {mut s:Chars}
    let mut ans: u128 = 0;
    s.reverse();
    for (i, &c) in s.iter().enumerate() {
        let bit = (c as u128 - 64) * 26_u128.pow(i as u32);
        ans += bit
    }
    println!("{}", ans)
}
