use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {n:usize,m:usize,edges:[(usize,usize);m]}

    let mut graph = UnionFind::<usize>::new(n);

    let mut ans = 0;
    for (a, b) in edges {
        let (a, b) = (a - 1, b - 1);
        if graph.equiv(a, b) {
            ans += 1;
        }
        graph.union(a, b);
    }
    println!("{}", ans)
}
