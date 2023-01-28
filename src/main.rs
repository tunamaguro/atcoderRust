use proconio::input;

fn main() {
    input! {n:usize,s:[String;n]}
    let mut a = 0;
    for si in s {
        if &si == "For" {
            a += 1;
        }
    }

    let ans = a >= (n / 2 + n % 2);
    println!("{}", if ans { "Yes" } else { "No" })
}
