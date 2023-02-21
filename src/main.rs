use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {n:usize,a:[i32;n]}
    let mut memo = HashMap::<&str, i32>::new();
    for ai in a {
        match ai {
            _ if ai < 400 => *memo.entry("hai").or_insert(0) += 1,
            _ if ai < 800 => *memo.entry("tya").or_insert(0) += 1,
            _ if ai < 1200 => *memo.entry("midori").or_insert(0) += 1,
            _ if ai < 1600 => *memo.entry("mizu").or_insert(0) += 1,
            _ if ai < 2000 => *memo.entry("ao").or_insert(0) += 1,
            _ if ai < 2400 => *memo.entry("ki").or_insert(0) += 1,
            _ if ai < 2800 => *memo.entry("daidai").or_insert(0) += 1,
            _ if ai < 3200 => *memo.entry("aka").or_insert(0) += 1,
            _ => *memo.entry("ziyu").or_insert(0) += 1,
        }
    }
    let ziyu = *memo.entry("ziyu").or_insert(0);
    let a = memo.keys().len() - 1;

    let l = if a == 0 { 1 } else { a };
    let g = a + ziyu as usize;
    println!("{} {}", l, g);
}
