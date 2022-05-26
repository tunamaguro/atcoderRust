use proconio::input;

fn main() {
    input! {n:usize,a:[i32;n]}
    let mut ans = a.iter().enumerate().collect::<Vec<_>>();
    ans.sort_by(|x, y| (-x.1).cmp(&(-y.1)));
    println!("{}", ans[1].0 + 1)
}
