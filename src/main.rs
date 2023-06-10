
use proconio::input;

fn main() {
    input! {n:i32}
    let water: Vec<_> = (0..21).map(|x| x * 5).collect();
    let ans = water.iter().fold(1000, |acc, x| {
        if (n - acc).abs() > (n - x).abs() {
            *x
        } else {
            acc
        }
    });
    println!("{}", ans)
}
