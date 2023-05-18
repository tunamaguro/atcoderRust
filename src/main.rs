use itertools::{self, Itertools};
use proconio::input;

fn main() {
    input! {n:usize}
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let t = n % (5 * 6);
    for i in 0..t {
        let a = i % 5;
        let b = a + 1;
        v.swap(a, b)
    }
    let ans = v.iter().map(|s| s.to_string()).join("");
    println!("{}", ans)
}
