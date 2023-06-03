use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n:usize,a:[(String,usize);n]}
    let m_idx = a.iter().position_min_by(|x, y| x.1.cmp(&y.1)).unwrap();
    for i in a.iter().take(n).skip(m_idx) {
        println!("{}", i.0)
    }
    for i in a.iter().take(m_idx) {
        println!("{}", i.0)
    }
}
