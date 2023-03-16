use proconio::input;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {n:usize,x:i32,a:[i32;n]}
    let a: Vec<_> = a.iter().map(|i| (i - x).abs()).collect();
    let mut ans = a[0];
    for i in a.iter().skip(1) {
        ans = gcd(ans, *i);
    }
    println!("{}", ans)
}
