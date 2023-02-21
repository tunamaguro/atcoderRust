use proconio::{input, marker::Chars};
use std::cmp::min;
fn main() {
    input! {h:usize,w:usize, s:[Chars;h]}
    let mut ans = vec![];
    for hi in 0..h {
        let mut r = vec![];
        for wi in 0..w {
            let hu = if hi == 0 { hi } else { hi - 1 };
            let hl = min(hi + 1, h - 1);
            let wl = if wi == 0 { wi } else { wi - 1 };
            let wr = min(wi + 1, w - 1);

            if s[hi][wi] == '#' {
                r.push('#'.to_string());
                continue;
            }

            let mut c = 0;
            for a in hu..=hl {
                for b in wl..=wr {
                    if a == hi && b == wi {
                        continue;
                    }
                    if s[a][b] == '#' {
                        c += 1;
                    }
                }
            }
            r.push(format!("{}", c))
        }
        ans.push(r);
    }

    for r in ans {
        println!("{}", r.join(""))
    }
}
