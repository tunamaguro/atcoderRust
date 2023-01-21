use proconio::{input, marker::Chars};

fn main() {
    input! {_n:usize,s:Chars}
    let mut ans: String = "".to_string();

    let mut nya = false;
    for si in s {
        match si {
            'n' => {
                nya = true;
                ans = format!("{}{}", ans, si.to_string());
            }
            'a' => {
                if nya {
                    ans = format!("{}{}", ans, "ya");
                } else {
                    ans = format!("{}{}", ans, si.to_string());
                }
                nya = false
            }
            _ => {
                nya = false;
                ans = format!("{}{}", ans, si.to_string());
            }
        }
    }

    println!("{}", ans)
}
