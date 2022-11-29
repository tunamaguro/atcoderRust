use proconio::input;

fn main() {
    input! {n:usize,a:[i128;n]}
    let c: i128 = 10_i128.pow(9) + 7;
    let mut sum = 0;
    for ai in &a {
        sum += ai;
        sum %= c;
    }
    let mut ans = 0;
    for ai in a {
        sum -= ai % c;
        sum %= c;
        if sum <= 0 {
            sum += c
        }
        ans += (sum * (ai % c)) % c;
        ans %= c;
    }
    println!("{}", ans)
}
