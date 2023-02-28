use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {h:usize,w:usize, a:[Chars;h]}
    let mut w_blank = vec![];

    for i in 0..h {
        let mut b = 0;
        for j in 0..w {
            if a[i][j] == '.' {
                b += 1;
            }
        }
        if b == w {
            w_blank.push(i);
        }
    }

    let mut h_blank = vec![];

    for j in 0..w {
        let mut b = 0;
        for i in 0..h {
            if a[i][j] == '.' {
                b += 1;
            }
        }
        if b == h {
            h_blank.push(j)
        }
    }

    let mut ans = vec![];
    for i in 0..h {
        if w_blank.contains(&i) {
            continue;
        }
        let mut r = vec![];
        for j in 0..w {
            if h_blank.contains(&j) {
                continue;
            }
            r.push(a[i][j])
        }
        ans.push(r);
    }
    let ans = ans.iter().map(|r| r.iter().map(|x| x.to_string()).join(""));
    for a in ans {
        println!("{}", a);
    }
}
