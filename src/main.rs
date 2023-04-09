use proconio::input;

fn main() {
    input! {n:usize,d:i32,t:[i32;n]}
    let mut ans = -1;
    t.iter().skip(1).fold(t[0], |acc, x| {
        if ans == -1 && x - acc <= d {
            ans = *x;
        }
        *x
    });

    println!("{}", ans)
}
