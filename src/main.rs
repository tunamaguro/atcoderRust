use proconio::input;

fn main() {
    input! {n:usize,k:usize,mut a:[i32;n]}
    a.sort();
    a.dedup();
    let mut ans = 0;
    for (i, &ai) in a.iter().enumerate() {
        if i != ai as usize {
            break;
        }
        if i >= k {
            break;
        }
        ans = ai + 1;
    }
    println!("{}", ans)
}
