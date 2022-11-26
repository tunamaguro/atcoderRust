use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars,t:Chars}
    let mut ans = "No";
    for i in 0..s.len() {
        let mut is_same = true;
        for (j, _) in t.iter().enumerate() {
            let si = i + j;
            if si >= s.len() {
                // tの探索範囲がsの長さを超えたらbreak
                is_same = false;
                break;
            }
            if s[si] != t[j] {
                is_same = false
            }
        }
        if is_same {
            ans = "Yes"
        }
    }
    println!("{}", ans)
}
