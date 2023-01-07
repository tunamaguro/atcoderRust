use std::vec;

use proconio::input;

fn dfs(graph: &Vec<Vec<bool>>, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for (i, e) in graph[v].iter().enumerate() {
        if !seen[i] && *e {
            dfs(graph, seen, i)
        }
    }
}

fn main() {
    input! {n:usize,m:usize}
    let mut edges = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {u:usize,v:usize}
        let (u, v) = (u - 1, v - 1);
        edges[u][v] = true;
        edges[v][u] = true;
    }
    let mut seen = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if !seen[i] {
            dfs(&edges, &mut seen, i);
            ans += 1;
        }
    }
    println!("{:?}", ans)
}
