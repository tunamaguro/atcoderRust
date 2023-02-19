use proconio::input;

fn main() {
    input! {n:usize,m:usize,a:[i32;n],b:[usize;m]}
    let mut ans = 0;

    for bi in b {
        ans += a[bi - 1]
    }

    println!("{}", ans)
}
