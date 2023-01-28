use proconio::input;

fn main() {
    input! {n:usize,m:usize,s:[i32;n],t:[i32;m]}
    let mut ans = 0;
    for si in s {
        let s1 = si % 10;
        let s2 = (si - s1) % 100;
        let s3 = (si - s1 - s2) % 1000;
        let a = [s3, s2, s1];
        for ti in &t {
            let t1 = ti % 10;
            let t2 = (ti - t1) % 100;
            let t3 = (ti - t1 - t2) % 1000;
            let b = [t3, t2, t1];
            if a == b {
                ans += 1;
                break;
            }
        }
    }

    println!("{}", ans)
}
