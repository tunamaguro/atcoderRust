use proconio::input;

fn main() {
    input! {n:i32}

    let mut ans = 0;
    for x in 1..n {
        let y = n - x;
        // println!("x = {} y = {}", x, y);
        let (mut a, mut b) = (0, 0);

        for i in 1..=x {
            if i * i > x {
                break;
            }
            if x % i == 0 {
                a += 1;
                if x != i * i {
                    a += 1;
                }
            }
        }

        for i in 1..=y {
            if i * i > y {
                break;
            }
            if y % i == 0 {
                b += 1;
                if y != i * i {
                    b += 1;
                }
            }
        }

        // println!("a = {} b = {}", a, b);

        ans += a * b;
    }
    println!("{}", ans)
}
