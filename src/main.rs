use proconio::input;

fn main() {
    input! {n:usize,mut a:[i32;n],q:usize}
    for _ in 0..q {
        input! {u:usize}
        match u {
            1 => {
                input! {k:usize,x:i32}
                a[k - 1] = x
            }
            2 => {
                input! {k:usize}
                println!("{}", a[k - 1])
            }
            _ => unreachable!(),
        }
    }
}
