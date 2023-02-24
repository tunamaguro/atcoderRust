use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {n:usize,mut a:[i32;n]}
    let mut memo = HashMap::new();
    for i in a {
        *memo.entry(i).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..(10_i32.pow(5)) {
        let total: i32 = (i..(i + 3)).map(|i| memo.get(&i).unwrap_or(&0)).sum();
        ans = std::cmp::max(ans, total);
    }

    println!("{}", ans)
}
