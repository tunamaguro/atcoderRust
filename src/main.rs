use proconio::input;

fn main() {
    input! {n:usize,a:usize,b:usize,c:[usize;n]}

    for (i, ci) in c.iter().enumerate() {
        if *ci == (a + b) {
            println!("{}", i + 1);
            return;
        }
    }
}
