use proconio::{input, marker::Chars};

fn main() {
    input! {_n:usize,mut s:Chars}
    let mut memo = Vec::<usize>::new();
    let mut toggle = false;
    for (i, si) in s.iter().enumerate() {
        if si == &'"' {
            toggle = !toggle;
        }
        if !toggle && si == &',' {
            memo.push(i)
        }
    }
    for i in memo {
        s[i] = '.';
    }
    let ans: String = s.iter().collect();
    println!("{}", ans)
}
