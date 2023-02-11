use proconio::input;

fn check(n: usize, t: Vec<&Vec<usize>>) -> bool {
    let mut all = vec![false; n];
    for a in t {
        for i in a {
            all[i - 1] = true;
        }
    }

    all.iter().all(|x| *x)
}

fn main() {
    input! {n:usize,m:usize}
    let mut s = vec![];
    for _ in 0..m {
        input! {c:usize,a:[usize;c]}
        s.push(a)
    }

    let mut ans = 0;
    for bit in 0..(1 << m) {
        let l: Vec<_> = (0..m)
            .filter(|x| bit & (1 << x) != 0)
            .map(|x| &s[x])
            .collect();

        if check(n, l) {
            ans += 1;
        }
    }

    println!("{}", ans)
}
