use proconio::input;

fn main() {
    input! {n:usize,x:i32,p:[i32;n]}
    for (i, pi) in p.iter().enumerate() {
        if pi == &x {
            println!("{}", i + 1)
        }
    }
}
