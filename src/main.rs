use proconio::input;

fn main() {
    input! {n:usize,m:usize}
    let mut memo = [[false; 105]; 105];
    for _ in 0..m {
        input! {k:usize,x:[usize;k]}
        for (i, x1) in x.iter().enumerate() {
            for x2 in x.iter().skip(i) {
                memo[*x1 - 1][*x2 - 1] = true;
                memo[*x2 - 1][*x1 - 1] = true;
            }
        }
    }
    let mut ans = true;
    for (n1, a) in memo.iter().enumerate() {
        for n2 in a.iter().take(n).skip(n1) {
            if !n2 {
                ans = false
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
