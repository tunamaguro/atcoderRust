use proconio::input;

fn main() {
    input! {n:f32}
    let maybe_ans = (n / 1.08).ceil();
    if (maybe_ans * 1.08).floor() == n {
        println!("{}", maybe_ans);
    } else {
        println!(":(");
    }
}
