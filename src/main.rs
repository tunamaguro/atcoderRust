use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {n:usize,mut x:[i32;5*n]}
    x.sort();
    let mut x = x.into_iter().collect::<VecDeque<_>>();
    for _ in 0..n {
        x.pop_back();
        x.pop_front();
    }
    let ans = x.iter().map(|&x| x as f64).sum::<f64>() / (3.0 * n as f64);
    println!("{}", ans)
}
