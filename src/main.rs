use proconio::{input, marker::Chars};
use std::cmp::max;

fn main() {
    input! {_n:usize,s:Chars}
    let mut ans = -1;
    let mut c = 0;
    let mut has_kushi = false;
    for i in s {
        match i {
            'o' => {
                c += 1;
                if has_kushi && c != 0 {
                    ans = max(ans, c);
                }
            }
            '-' => {
                has_kushi = true;
                if has_kushi && c != 0 {
                    ans = max(ans, c);
                }
                c = 0;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans)
}
