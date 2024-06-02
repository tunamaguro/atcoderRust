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
    input! {n:usize,m:usize,a:[i32;m],x:[[i32;m];n]}

    let mut eiyou = vec![0; m];
    for xi in x {
        for (i, x_ij) in xi.iter().enumerate() {
            *eiyou.get_mut(i).unwrap() += x_ij;
        }
    }

    let mut ans = true;
    for i in 0..m {
        ans &= eiyou[i] >= a[i];
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
