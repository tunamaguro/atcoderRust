use proconio::input;

fn main() {
    input! {n:usize,x:usize,a:[usize;n]}
    let mut ans = false;
    for ai in a {
        if ai == x {
            ans = true
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
