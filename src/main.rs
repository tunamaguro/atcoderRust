use proconio::{input, marker::Chars};

fn main() {
    input! {h:usize,w:usize,s:[Chars;h]}
    for hi in 0..h {
        for wi in 0..w {
            if s[hi][wi] != '.' {
                continue;
            }

            let mut c = 0;
            if hi > 0 && s[hi - 1][wi] == '#' {
                c += 1;
            }
            if hi < h - 1 && s[hi + 1][wi] == '#' {
                c += 1;
            }
            if wi > 0 && s[hi][wi - 1] == '#' {
                c += 1;
            }
            if wi < w - 1 && s[hi][wi + 1] == '#' {
                c += 1;
            }
            if c >= 2 {
                println!("{} {}", hi + 1, wi + 1);
                return;
            }
        }
    }
}
