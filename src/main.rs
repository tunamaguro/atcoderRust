use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let mut wb = 0;
    let mut bw = 0;
    for (i, &c) in s.iter().enumerate() {
        match (i % 2, c) {
            (0, '0') => {
                wb += 1;
            }
            (1, '0') => {
                bw += 1;
            }
            (0, '1') => {
                bw += 1;
            }
            (1, '1') => {
                wb += 1;
            }
            _ => unreachable!(),
        }
    }
    let ans = std::cmp::min(wb, bw);
    println!("{}", ans)
}
