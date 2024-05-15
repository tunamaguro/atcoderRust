use itertools::Itertools;
use proconio::input;

trait Bound<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: PartialOrd> Bound<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let (mut low, mut high) = (0, self.len());
        while low + 1 < high {
            let mid = (low + high) / 2;
            if self[mid] < *x {
                low = mid;
            } else {
                high = mid;
            }
        }
        if self[low] < *x {
            low + 1
        } else {
            low
        }
    }

    fn upper_bound(&self, x: &T) -> usize {
        let (mut low, mut high) = (0, self.len());
        while low + 1 < high {
            let mid = (low + high) / 2;
            if self[mid] <= *x {
                low = mid;
            } else {
                high = mid;
            }
        }
        if self[low] <= *x {
            low + 1
        } else {
            low
        }
    }
}

fn main() {
    input! {n:usize,a:[usize;n]}
    let a = a.into_iter().sorted().collect_vec();
    let mut acc = vec![0];
    for ai in &a {
        acc.push(ai + acc.last().unwrap());
    }

    let e = 10_usize.pow(8);

    let mut ans = 0;
    for i in 0..n - 1 {
        let ai = a[i];
        let ajs = &a[(i + 1)..];
        ans += ai * ajs.len();
        ans += acc.last().unwrap() - acc[i + 1];
        // println!("ans = {}", ans);
        if ai < e {
            let lower_10e8 = ajs.lower_bound(&(e - ai));
            // #[cfg(debug_assertions)]
            // {
            //     println!("ai: {}, ajs: {:?}, lower_10e8: {}", ai, ajs, lower_10e8);
            //     println!(
            //         "ajs.len: {}, num of over 10e8(a+b): {}",
            //         ajs.len(),
            //         ajs.len() - lower_10e8
            //     );
            // }
            ans -= e * (ajs.len() - lower_10e8)
        } else {
            ans -= e * ajs.len();
        }
    }
    println!("{}", ans);
}
