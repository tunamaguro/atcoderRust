use proconio::input;

fn main() {
    input! {n:usize,h:[i32;n]}
    let ans = h.iter().enumerate().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    println!("{}", ans.0 + 1)
}
