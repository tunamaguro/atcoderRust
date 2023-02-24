use proconio::input;

fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn main() {
    input! {n:usize,m:usize,s:[(i32,i32);n],g:[(i32,i32);m]}
    let ans = s
        .iter()
        .map(|&p1| g.iter().map(move |&p2| manhattan(p1, p2)))
        .map(|x| x.collect::<Vec<_>>())
        // .map(|x| x.rev().collect::<Vec<_>>())
        // .map(|x| *x.iter().min_by_key(|&d| d).unwrap())
        .collect::<Vec<_>>();

    for i in 0..n {
        let m = ans[i].iter().min().unwrap();
        let id = ans[i].iter().position(|x| x == m).unwrap() + 1;
        println!("{}", id)
    }
}
