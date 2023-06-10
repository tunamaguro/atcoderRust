use proconio::input;

fn main() {
    input! {p:char,q:char}
    let mut a = [p, q];
    a.sort();
    let [p, q] = a;
    let alpha: Vec<_> = "ABCDEFG".chars().collect();
    let distance = [3, 1, 4, 1, 5, 9];
    let pi = alpha.iter().position(|&x| x == p).unwrap();
    let qi = alpha.iter().position(|&x| x == q).unwrap();
    let ans: i32 = distance[pi..qi].iter().sum();
    println!("{}", ans);
}
