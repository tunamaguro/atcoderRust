use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {k:usize}
    let mut queue = VecDeque::<usize>::new();
    for i in 1..=9 {
        queue.push_back(i)
    }

    for _ in 0..k - 1 {
        let a = queue.pop_front().unwrap();
        let l = a % 10;
        if l == 0 {
            queue.push_back(10 * a + l);
            queue.push_back(10 * a + l + 1);
        } else if l == 9 {
            queue.push_back(10 * a + l - 1);
            queue.push_back(10 * a + l);
        } else {
            queue.push_back(10 * a + l - 1);
            queue.push_back(10 * a + l);
            queue.push_back(10 * a + l + 1);
        }
    }

    println!("{}", queue.pop_front().unwrap())
}
