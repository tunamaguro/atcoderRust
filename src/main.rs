use proconio::input;

fn main() {
    input! {mut h:i32,mut m:i32}
    loop {
        let h2 = h / 10;
        let h1 = h % 10;
        let m2 = m / 10;
        let m1 = m % 10;

        let miss_h = h2 * 10 + m2;
        let miss_m = h1 * 10 + m1;
        if miss_h <= 23 && miss_m <= 59 {
            break;
        }
        m += 1;
        if m > 59 {
            h += 1;
            m = 0;
        }
        if h > 23 {
            h = 0
        }
    }
    println!("{} {}", h, m)
}
