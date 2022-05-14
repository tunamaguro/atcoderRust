use proconio::input;

fn main() {
    input! {mut n:i32,k:i32}
    let mut digit = 0;
    while n != 0 {
        n /= k;
        digit += 1;
    }
    println!("{}", digit)
}
