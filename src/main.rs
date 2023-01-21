use proconio::input;

fn main() {
    input! {n:usize,p:usize,q:usize,r:usize,_s:usize,mut a:[i32;n]}
    let (p, q, r) = (p - 1, q - 1, r - 1);
    for i in 0..=(q - p) {
        let t = a[p + i];
        a[p + i] = a[r + i];
        a[r + i] = t;
    }
    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
