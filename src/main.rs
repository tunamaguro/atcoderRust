use proconio::input;

fn main() {
    input! {n:usize,p:[(i32,i32);n]}
    for (a, b) in p {
        println!("{}", a + b)
    }
}
