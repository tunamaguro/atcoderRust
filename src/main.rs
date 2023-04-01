use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {n:usize,x:i32,a:[i32;n]}
    let mut memo = HashMap::new();
    for i in a {
        *memo.entry(i).or_insert(0) += 1
    }
    let mut ans = false;
    for ai in memo.keys() {
        let aj = -(x - ai);
        let has_pair = memo.contains_key(&aj);
        ans |= has_pair
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
