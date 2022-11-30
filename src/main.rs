use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {_n:usize,q:usize}
    let mut memo = HashSet::<(usize, usize)>::new();
    for _ in 0..q {
        input! {t:usize,a:usize,b:usize}
        match t {
            1 => {
                memo.insert((a, b));
            }
            2 => {
                memo.remove(&(a, b));
            }
            3 => {
                if memo.contains(&(a, b)) && memo.contains(&(b, a)) {
                    println!("Yes")
                } else {
                    println!("No")
                }
            }
            _ => unreachable!(),
        }
    }
}
