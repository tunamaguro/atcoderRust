use proconio::input;

fn main() {
    input! {n:usize,a:[usize;n]}
    let mut memo = vec![0];
    for ai in a {
        let ai = ai - 1;
        let generation = memo[ai] + 1;
        memo.push(generation);
        memo.push(generation);
    }
    for g in memo {
        println!("{}", g)
    }
}
