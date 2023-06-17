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
    input! {n:usize,a:[usize;3*n]}
    let mut c = vec![0; n];
    let mut ans = vec![0; n];
    for (i, &ai) in a.iter().enumerate() {
        let ai = ai - 1;
        c[ai] += 1;
        if c[ai] == 2 {
            ans[ai] = i + 1;
        }
    }
    let ans = ans
        .iter()
        .enumerate()
        .sorted_by(|a, b| a.1.cmp(b.1))
        .map(|x| x.0 + 1);
    let ans = ans.map(|x| x.to_string()).join(" ");
    println!("{}", ans)
}
