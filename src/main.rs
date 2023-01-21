use proconio::input;

fn main() {
    input! {a:usize,b:usize}
    println!("{}", a.pow(std::convert::TryInto::try_into(b).unwrap()))
}
