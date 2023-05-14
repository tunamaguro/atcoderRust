use proconio::{input, marker::Chars};

fn main() {
    input! {n:usize,s:Chars}
    let mut t = 0;
    let mut a = 0;
    let half = n / 2 + n % 2;
    for si in s {
        match si {
            'T' => t += 1,
            'A' => a += 1,
            _ => unreachable!(),
        }
        if t >= half {
            println!("T");
            break;
        }
        if a >= half {
            println!("A");
            break;
        }
    }
}
