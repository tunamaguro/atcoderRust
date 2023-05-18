use itertools::{self, Itertools};
use proconio::input;
fn main() {
    input! {h:usize,w:usize,a:usize,b:usize}
    let mut ans = vec![];
    for hi in 1..=h {
        let mut s = vec![];
        for wi in 1..=w {
            if hi <= b {
                if wi <= a {
                    s.push(0)
                } else {
                    s.push(1)
                }
            } else {
                if wi <= a {
                    s.push(1)
                } else {
                    s.push(0)
                }
            }
        }
        ans.push(s.iter().map(|x| x.to_string()).join(""))
    }
    for i in ans {
        println!("{}", i)
    }
}
