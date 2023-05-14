use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n:usize,a:[usize;n]}
    let mut ans = vec![];
    ans.push(a[0]);
    for &ai in a.iter().skip(1) {
        let l = *ans.last().unwrap();
        for i in (l..=ai).skip(1) {
            ans.push(i);
        }
        for i in (ai..l).rev() {
            ans.push(i);
        }
    }
    println!("{}", ans.iter().join(" "))
}
