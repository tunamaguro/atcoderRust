use proconio::{input, marker::Chars};

fn main() {
    input! {h:usize,_w:usize,s:[Chars;h]}
    let mut ans = 0;
    for si in s {
        for c in si {
            if c == '#' {
                ans += 1
            }
        }
    }
    println!("{}", ans)
}
