use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n:usize,t:usize,c:[usize;n],r:[usize;n]}
    let z = c
        .iter()
        .enumerate()
        .zip(r.iter())
        .map(|((i, c), r)| (i + 1, c, r));
    // 色Tのカードが存在するとき
    let a = z
        .clone()
        .filter(|(_, &c, _)| c == t)
        .sorted_by(|a, b| a.2.cmp(b.2))
        .rev()
        .next();
    if let Some(x) = a {
        println!("{}", x.0);
        return;
    }

    // 色Tのカードが存在しないとき
    let r = z
        .filter(|(_, &x, _)| x == c[0])
        .sorted_by(|a, b| a.2.cmp(b.2))
        .rev()
        .next();
    if let Some(x) = r {
        println!("{}", x.0)
    }
}
