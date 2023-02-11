use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let ans = s
        .iter()
        .map(|&c| {
            if c == '0' {
                '1'.to_string()
            } else {
                '0'.to_string()
            }
        })
        .collect::<Vec<_>>();

    let ans = ans.join("");

    println!("{}", ans)
}
