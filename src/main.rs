use proconio::{input, marker::Chars};
use std::cmp::min;
use std::collections::VecDeque;

fn check_rotate(s: &VecDeque<&char>) -> u128 {
    let n = s.len();
    let mut ans = 0;
    for i in 0..(n / 2 + n % 2) {
        if s[i] != s[n - i - 1] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    input! {n:usize,a:u128,b:u128,mut s:Chars}
    let mut s = s.iter().collect::<VecDeque<_>>();
    let mut ans: u128 = check_rotate(&s) * b;
    for i in 1..=n {
        let t = s.pop_front().unwrap();
        s.push_back(t);

        ans = min(ans, i as u128 * a + check_rotate(&s) * b)
    }
    println!("{}", ans)
}
