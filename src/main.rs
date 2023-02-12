use proconio::input;

fn main() {
    input! {n:usize,a:[usize;n],m:usize,b:[usize;m],x:usize}

    let mut memo = vec![false; x + 1];
    let mut mochi = vec![false; x + 1];
    for bi in b {
        mochi[bi] = true;
    }
    memo[0] = true;
    for i in 0..x {
        if mochi[i] {
            continue;
        }

        if !memo[i] {
            continue;
        }

        for &ai in &a {
            if i + ai <= x {
                memo[i + ai] = true;
            }
        }
    }

    println!("{}", if memo[x] { "Yes" } else { "No" })
}
