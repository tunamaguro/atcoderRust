use proconio::input;
use std::collections::{self, HashMap};

fn dfs(i: usize, memo: &HashMap<usize, usize>, v: &mut Vec<usize>, seen: &mut Vec<bool>) {
    seen[i - 1] = true;
    if let Some(to) = memo.get(&i) {
        dfs(*to, memo, v, seen)
    }
    v.push(i);
}

fn main() {
    input! {n:usize,m:usize,a:[usize;m]}
    let mut memo = collections::HashMap::<usize, usize>::new();
    for ai in a {
        memo.insert(ai, ai + 1);
    }

    let mut ans = vec![];
    let mut seen = vec![false; n];
    for i in 1..=n {
        if seen[i - 1] {
            continue;
        }
        dfs(i, &memo, &mut ans, &mut seen)
    }
    println!(
        "{}",
        ans.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
