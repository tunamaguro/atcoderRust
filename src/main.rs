use proconio::input;

// ka=c mod b
// ka-c=lm

fn main() {
    input! {a:i32,b:i32,c:i32}
    let ans = (0..110).map(|x| x * a).any(|x| x % b == c);

    println!("{}", if ans { "YES" } else { "NO" })
}
