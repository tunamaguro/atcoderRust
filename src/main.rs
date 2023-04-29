use proconio::{input, marker::Chars};

fn main() {
    input! {h:usize,w:usize, a:[Chars;h],b:[Chars;h]}
    for si in 0..h {
        for ti in 0..w {
            let mut a = a.clone();
            // 縦方向シフト
            for _ in 0..si {
                for wi in 0..w {
                    let r0 = a[0][wi];
                    for hi in 0..(h - 1) {
                        a[hi][wi] = a[hi + 1][wi]
                    }
                    a[h - 1][wi] = r0
                }
            }

            // 横方向シフト
            for _ in 0..ti {
                for hi in 0..h {
                    let c0 = a[hi][0];
                    for wi in 0..(w - 1) {
                        a[hi][wi] = a[hi][wi + 1];
                    }
                    a[hi][w - 1] = c0
                }
            }
            if a == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No")
}
