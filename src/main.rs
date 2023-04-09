use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let mut ans = true;

    let mut b1 = -1;
    for (i, c) in s.iter().enumerate() {
        if b1 != -1 && c == &'B' {
            let x = b1;
            let y = i as i32;
            ans &= x % 2 != y % 2;
            break;
        }
        if b1 == -1 && c == &'B' {
            b1 = i as i32;
        }
    }

    let mut r = 0;
    for c in s {
        if c == 'K' && r != 1 {
            ans = false;
            break;
        }
        if c == 'R' {
            r += 1;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" })
}
