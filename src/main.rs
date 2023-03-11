use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {n:usize,a:[usize;n]}
    let mut memo = HashSet::new();
    for i in 1..=n {
        memo.insert(i);
    }
    for (i, ai) in a.iter().enumerate() {
        let i = i + 1;
        if memo.contains(&i) {
            memo.remove(ai);
        }
    }

    let mut ans: Vec<_> = memo.iter().collect();
    println!("{}", ans.len());
    ans.sort();
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
