use proconio::input;

fn main() {
    input! {n:i32}
    if n < 10_i32.pow(3) {
        println!("{}", n)
    } else if n < 10_i32.pow(4) {
        println!("{}", n - n % 10_i32.pow(1))
    } else if n < 10_i32.pow(5) {
        println!("{}", n - n % 10_i32.pow(2))
    } else if n < 10_i32.pow(6) {
        println!("{}", n - n % 10_i32.pow(3))
    } else if n < 10_i32.pow(7) {
        println!("{}", n - n % 10_i32.pow(4))
    } else if n < 10_i32.pow(8) {
        println!("{}", n - n % 10_i32.pow(5))
    } else if n < 10_i32.pow(9) {
        println!("{}", n - n % 10_i32.pow(6))
    }
}
