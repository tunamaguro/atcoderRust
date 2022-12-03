use proconio::input;
use std::collections::HashMap;

fn prime_factorization(n: i128) -> HashMap<i128, i128> {
    let mut ans = HashMap::new();
    let mut n = n;

    let mut p: i128 = 2;
    while p * p <= n {
        if n % p != 0 {
            p += 1;
            continue;
        }
        let mut exponent = 0;
        while n % p == 0 {
            exponent += 1;
            n /= p;
        }
        ans.insert(p, exponent);
    }
    if n != 1 {
        ans.insert(n, 1);
    }

    ans
}

fn is_multiple(n: i128, m: &HashMap<i128, i128>) -> bool {
    // 因数をすべてn!が持っていたらtrue,そうでなければfalse
    let mut ok = true;
    for (k, size) in m {
        let mut count_div = 0;
        let p = *k;
        let mut e = 1;
        while p.pow(e) <= n {
            count_div += n / p.pow(e);
            e += 1;
        }
        ok = ok && (count_div >= *size)
    }

    ok
}

fn main() {
    // 理論及び実装参考
    // https://qiita.com/drken/items/97e37dd6143e33a64c8c
    // https://mathscience-teach.com/koukoumath-seisuu1-5/
    // https://algo-method.com/descriptions/119
    input! {k:i128}
    let a = prime_factorization(k);

    let mut left: i128 = -1;
    let mut right = k;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if is_multiple(mid, &a) {
            right = mid
        } else {
            left = mid
        }
    }
    println!("{}", right)
}
