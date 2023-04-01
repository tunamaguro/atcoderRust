use proconio::{input, marker::Chars};

fn main() {
    input! {board:[Chars;8]}
    let col = (0..8).map(|x| (x + b'a') as char).collect::<Vec<_>>();
    let row = (1..=8).rev().collect::<Vec<_>>();
    for (i, r) in board.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == '*' {
                println!("{}{}", col[j], row[i])
            }
        }
    }
}
