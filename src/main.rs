use proconio::input;

fn main() {
    input! {n:usize,m:usize,c:i32,b:[i32;m], codes:[[i32;m];n]}
    let mut ans = 0;
    for code in codes {
        let tmp: i32 = code.iter().zip(b.iter()).map(|(a, b)| a * b).sum();
        if tmp + c > 0 {
            ans += 1;
        }
    }
    println!("{}", ans)
}
