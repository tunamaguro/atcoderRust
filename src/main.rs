use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {n:usize,s:[Chars;n]}
    let mut l: Vec<usize> = Vec::new();
    for i in (0..=9).map(|x| std::char::from_digit(x as u32, 10).unwrap()) {
        let mut map = HashMap::<usize, usize>::new();
        s.iter()
            .map(|c| c.iter().position(|x| x == &i).unwrap())
            .for_each(|x| *map.entry(x).or_insert(0) += 1);
        let map_vec: Vec<_> = map.into_iter().collect();
        let max = map_vec
            .iter()
            .max_by(|x, y| {
                if x.1 == y.1 {
                    return x.0.cmp(&y.0);
                }
                x.1.cmp(&y.1)
            })
            .unwrap();
        l.push(max.0 + 10 * (max.1 - 1))
    }
    let ans = l.iter().min().unwrap();
    println!("{}", ans)
}
