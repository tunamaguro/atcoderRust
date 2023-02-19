use proconio::{input, marker::Chars};

fn main() {
    input! {_n:usize,k:usize,s:Chars}
    let mut ans = String::new();
    let mut c = 0;
    for si in s {
        if si == 'o' && c < k {
            ans.push('o');
            c += 1;
        } else {
            ans.push('x');
        }
    }
    println!("{}", ans)
}
