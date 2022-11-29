use proconio::input;

fn main() {
    input! {a:i128,b:i128,c:i128}
    let d = 10_i128.pow(9) + 7;
    let ans = (a % d) * (b % d) * (c % d);
    let ans = ans % d;
    println!("{}", ans)
}
