use proconio::input;

fn main() {
    input! {h:i128,w:i128}
    let mut ans = h * w / 2 + (h * w) % 2;
    if h == 1 || w == 1 {
        ans = 1;
    }
    println!("{}", ans)
}
