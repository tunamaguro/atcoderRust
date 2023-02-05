use proconio::{input, marker::Chars};

fn main() {
    input! {_n:usize,a:i32,b:i32,s:Chars}

    let mut a_num = 0;
    let mut b_num = 0;

    for si in s {
        let tmp = (a + b) > (a_num + b_num);
        match si {
            'a' => {
                if tmp {
                    println!("Yes");
                    a_num += 1;
                } else {
                    println!("No");
                }
            }
            'b' => {
                if tmp && b > b_num {
                    println!("Yes");
                    b_num += 1;
                } else {
                    println!("No");
                }
            }
            _ => {
                println!("No");
            }
        }
    }
}
