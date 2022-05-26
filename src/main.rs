use proconio::input;

fn main() {
    input! {_h:usize,_w:usize,n:usize,point:[(usize,usize);n]}
    let mut x_blank = point.iter().map(|v| v.0).collect::<Vec<_>>();
    let mut y_blank = point.iter().map(|v| v.1).collect::<Vec<_>>();
    x_blank.sort();
    y_blank.sort();
    x_blank.dedup();
    y_blank.dedup();
    for (x, y) in point {
        let x_pos = x_blank.binary_search(&x).unwrap() + 1;
        let y_pos = y_blank.binary_search(&y).unwrap() + 1;
        println!("{} {}", &x_pos, &y_pos)
    }
}
