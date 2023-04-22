use proconio::input;

use proconio::source::line::LineSource;
use std::io;
use std::io::BufReader;

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {n:i32}
    let mut left = 0;
    let mut right = n;
    for _ in 0..20 {
        let mid = left + (right - left) / 2;
        if right - left > 1 {
            println!("? {}", mid);
            input! {s:i32}
            match s {
                0 => left = mid,
                1 => right = mid,
                _ => unreachable!(),
            }
        } else {
            println!("! {}", left);
            break;
        }
    }
}
