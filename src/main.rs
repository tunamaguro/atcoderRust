use proconio::input;

fn main() {
    input! {n:usize,mut k:i64,mut h:[i64;n]}
    h.sort();
    h.reverse();
    for i in h.iter_mut() {
        if k == 0 {
            break;
        }
        *i = 0;
        k -= 1;
    }
    let ans: i64 = h.iter().sum();
    println!("{}", ans)
}
