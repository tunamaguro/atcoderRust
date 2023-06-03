use proconio::input;

fn main() {
    input! {n:usize,d:i32,p:[(i32,i32);n]}
    let mut has_infected = vec![false; n];
    let mut queue: Vec<usize> = vec![0];
    while !queue.is_empty() {
        let i = queue.pop().unwrap();
        if has_infected[i] {
            continue;
        }
        has_infected[i] = true;
        let (x1, y1) = p[i];
        for (i, (x2, y2)) in p.iter().enumerate() {
            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);
            if d.pow(2) >= dist {
                queue.push(i)
            }
        }
    }
    for ans in has_infected {
        println!("{}", if ans { "Yes" } else { "No" })
    }
}
