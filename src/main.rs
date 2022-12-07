use proconio::input;
use std::cmp::max;

fn main() {
    input! {n:usize,a:[i32;n]}
    let mut even = vec![];
    let mut odd = vec![];
    for ai in a {
        if ai % 2 == 0 {
            even.push(ai)
        } else {
            odd.push(ai)
        }
    }
    even.sort();
    odd.sort();
    even.reverse();
    odd.reverse();

    let mut ans = -1;

    if even.len() >= 2 {
        ans = max(ans, even[0] + even[1])
    }
    if odd.len() >= 2 {
        ans = max(ans, odd[0] + odd[1])
    }
    println!("{}", ans)
}
