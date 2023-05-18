use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};
fn main() {
    input! {q:usize}
    // println!("---debug---");
    let mut a = VecDeque::new();
    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input! {t:usize}
        match t {
            1 => {
                input! {x:i32}
                a.push_back(x);
            }
            2 => {
                if heap.is_empty() {
                    let x = a.pop_front().unwrap();
                    println!("{}", x)
                } else {
                    let x: Reverse<i32> = heap.pop().unwrap();
                    println!("{}", x.0)
                }
            }
            3 => {
                for i in &a {
                    heap.push(Reverse(*i));
                }
                a.clear();
            }
            _ => unreachable!(),
        }
    }
}
