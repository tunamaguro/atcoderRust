use proconio::input;

fn main() {
    input! {a:[[i32;3];3],n:usize,b:[i32;n]}

    let mut bingo = [[false; 3]; 3];

    for i in b {
        for h in 0..3 {
            for w in 0..3 {
                bingo[h][w] |= a[h][w] == i
            }
        }
    }

    let mut ans = false;
    for i in 0..3 {
        let w_bingo = bingo[i].iter().all(|x| *x);
        let h_bingo = bingo.iter().map(|w| w[i]).all(|x| x);
        ans |= w_bingo || h_bingo;
    }
    let rd_bingo = (0..3).map(|i| bingo[i][i]).all(|x| x);
    let ld_bingo = (0..3)
        .into_iter()
        .zip((0..3).rev())
        .map(|(h, w)| bingo[h][w])
        .all(|x| x);
    ans |= rd_bingo || ld_bingo;
    println!("{}", if ans { "Yes" } else { "No" })
}
