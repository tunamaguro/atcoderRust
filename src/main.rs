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

fn recursive(l: u64, r: u64, org_l: u64, org_r: u64, dst: &mut Vec<(u64, u64)>) {
    if org_l <= l && r <= org_r {
        dst.push((l, r));
        return;
    }
    let mid = (l + r) / 2;
    if mid <= org_l {
        recursive(mid, r, org_l, org_r, dst);
    } else if org_r <= mid {
        recursive(l, mid, org_l, org_r, dst);
    } else {
        recursive(l, mid, org_l, org_r, dst);
        recursive(mid, r, org_l, org_r, dst);
    }
}

fn main() {
    input! {mut l:u64,mut r:u64}
    let mut ans: Vec<(u64, u64)> = vec![];
    recursive(0, 2_u64.pow(60), l, r, &mut ans);
    println!("{}", ans.len());
    for (li, ri) in ans {
        println!("{} {}", li, ri);
    }
}
