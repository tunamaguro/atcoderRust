use proconio::{input, marker::Chars};

fn main() {
    input! {mut s:Chars}
    let mut ans = 2;
    for _ in 0..s.len() {
        s.pop();
        let si = s.len();
        if si % 2 == 0 {
            let si = si / 2;
            let s1 = &s[..si];
            let s2 = &s[si..];
            if s1 == s2 {
                ans = std::cmp::max(ans, si * 2);
            }
        }
    }
    println!("{}", ans)
}
