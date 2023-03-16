use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {n:usize,h:[i32;n]}
    // 0="=" 1="<" -1=">"
    let mut a = 0;
    let mut b = h[0];
    let mut memo = vec![h[0]];
    for i in h {
        match i.cmp(&b) {
            Ordering::Equal => {}
            Ordering::Greater => {
                if a == -1 {
                    memo.push(b);
                    memo.push(b);
                }
                a = 1;
                *memo.last_mut().unwrap() = i;
            }
            Ordering::Less => a = -1,
        }

        b = i;
    }
    let mut ans = 0;
    let mut i = 0;
    for hi in memo {
        if hi >= i {
            ans += (hi - i).abs();
        }
        i = hi;
    }
    println!("{}", ans)
}
