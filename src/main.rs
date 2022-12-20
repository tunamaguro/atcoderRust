use proconio::input;

fn main() {
    input! {k:usize}
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let ans: String = alphabet.take(k).collect();
    println!("{}", ans)
}
