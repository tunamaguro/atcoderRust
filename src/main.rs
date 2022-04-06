use proconio::input;
use std::cmp::min;

fn main() {
    input! {k:usize,n:usize,mut a:[usize;n]}
    let mut ans=a.iter().max().unwrap()-a.iter().min().unwrap();
    let mut house:Vec<_>=vec![0];
    house.append(&mut a);
    house.push(k);
    for i in 1..=n{
        let b=house[i-1];
        let a=house[i];
        ans=min(ans,k-(a-b))
    }
    println!("{}",ans)
}
