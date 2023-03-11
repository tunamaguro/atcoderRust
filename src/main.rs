use proconio::input;

fn check(board: &Vec<Vec<i32>>, ans: &mut i32, x: usize, y: usize, mut memo: Vec<i32>) {
    // println!("x = {} y = {}", x, y);

    let now = board[y][x];
    for i in &memo {
        if *i == now {
            return;
        }
    }

    if y == board.len() - 1 && x == board[0].len() - 1 {
        *ans += 1;
        return;
    }

    memo.push(now);
    let rd = [(1, 0), (0, 1)];
    for (xp, yp) in &rd {
        let xn = x + xp;
        let yn = y + yp;
        if xn < board[0].len() && yn < board.len() {
            check(board, ans, xn, yn, memo.clone())
        }
    }
}

fn main() {
    input! {h:usize,w:usize,a:[[i32;w];h]}

    let mut ans = 0;
    check(&a, &mut ans, 0, 0, vec![]);
    println!("{}", ans)
}
