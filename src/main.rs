use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let mut ans = 0;
    let mut white_count = 0;
    for (idx, si) in s.iter().enumerate() {
        if si == &'W' {
            ans += idx - white_count;
            white_count += 1;
        }
    }
    println!("{}", ans)
}
