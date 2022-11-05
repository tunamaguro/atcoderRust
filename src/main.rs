use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {n:usize,m:usize,edges:[(usize,usize);m]}
    let mut memo = HashMap::<usize, Vec<usize>>::new();
    for (ai, bi) in edges {
        let a = memo.entry(ai).or_insert(vec![]);
        a.push(bi);
        let b = memo.entry(bi).or_insert(vec![]);
        b.push(ai);
    }

    for i in 1..=n {
        let mut connect_city: Vec<usize> = match memo.get(&i) {
            Some(arr) => arr.to_vec(),
            None => Vec::<usize>::new(),
        };
        let mut ans = vec![connect_city.len()];
        connect_city.sort();
        ans.append(&mut connect_city);
        let ans = ans
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans)
    }
}
