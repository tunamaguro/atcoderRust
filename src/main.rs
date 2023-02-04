use proconio::input;

fn dfs(g: &Vec<Vec<bool>>, seen: &mut Vec<bool>, to: usize) {
    seen[to] = true;
    for (next, conn) in g[to].iter().enumerate() {
        if !conn {
            continue;
        }
        if seen[next] {
            continue;
        }

        dfs(g, seen, next);
    }
}

fn main() {
    input! {n:usize,m:usize,edges:[(usize,usize);m]}
    let mut graph = vec![vec![false; n]; n];
    for (u, v) in &edges {
        let u = u - 1;
        let v = v - 1;
        graph[u][v] = true;
        graph[v][u] = true;
    }
    let mut seen = vec![false; n];

    let mut c = 0;

    for i in 0..n {
        if seen[i] {
            continue;
        }
        dfs(&graph, &mut seen, i);
        c += 1;
    }

    let ans = c == 1
        && edges.len() == (n - 1)
        && graph
            .iter()
            .map(|x| x.iter().filter(|x| **x).count())
            .all(|x| x <= 2);

    println!("{}", if ans { "Yes" } else { "No" })
}
