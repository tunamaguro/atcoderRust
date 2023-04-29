use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::cmp::min;

fn main() {
    input! {h:usize,w:usize,c:[Chars;h]}
    let mut ans = vec![0; min(h, w)];
    let mut visited = vec![vec![false; w]; h];

    let mut queue: Vec<(usize, usize)> = vec![];

    for hi in 0..h {
        for wi in 0..w {
            // すでに訪れていればスキップ
            if visited[hi][wi] {
                continue;
            }
            if c[hi][wi] != '#' {
                visited[hi][wi] = true;
                continue;
            }
            let mut edge = 0;
            queue.push((hi, wi));
            while !queue.is_empty() {
                let (nh, nw) = queue.pop().unwrap();
                // println!(
                //     "now ({} , {})  visited = {}  queue {:?}",
                //     nh, nw, visited[nh][nw], queue
                // );
                if visited[nh][nw] {
                    continue;
                }
                visited[nh][nw] = true;

                if c[nh][nw] != '#' {
                    continue;
                }
                edge += 1;

                // 斜めに探索する
                // 左上
                if 0 < nh && 0 < nw {
                    queue.push((nh - 1, nw - 1))
                }
                // 右上
                if 0 < nh && nw < w - 1 {
                    queue.push((nh - 1, nw + 1))
                }
                // 左下
                if nh < h - 1 && 0 < nw {
                    queue.push((nh + 1, nw - 1))
                }
                // 右下
                if nh < h - 1 && nw < w - 1 {
                    queue.push((nh + 1, nw + 1))
                }
                // println!("edge = {}", edge)
            }
            let batsu = (edge - 1) / 4;
            ans[batsu - 1] += 1;
            // println!("batsu size = {}", batsu)
        }
    }

    let ans = ans.iter().map(|x| x.to_string()).join(" ");
    println!("{}", ans)
}
