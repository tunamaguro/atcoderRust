use proconio::input;

// See mPn and mCn
// https://manabitimes.jp/math/1352
fn p(m:i128,n:i128)->i128{
    let mut s=m;
    for i in 1..n{
        s*=m-i;
    }
    s
}

fn f(n:i128)->i128{
    if n==0{
        1
    }else{
        f(n-1)*n
    }
}

fn main() {
    input! {l:i128}
    let ans=p(l-1,11)/f(11);
    println!("{}",ans)
}
