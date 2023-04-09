use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {a:i128,b:i128}
    let mut ans = 0;
    let mut x = max(a, b);
    let mut y = min(a, b);
    // println!("x = {}  y = {}", x, y);
    while x != y {
        // println!("x = {}  y = {}  ans = {}", x, y, ans);
        let div = x / y;
        ans += div;
        let a = x % y;
        let b = y;
        if a == 0 {
            ans -= 1;
            break;
        }
        x = max(a, b);
        y = min(a, b);
    }
    println!("{}", ans)
}
