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
    input! {n:usize,m:usize,mut takahashi:[(usize,usize);m],mut aoki:[(usize,usize);m]}
    let mut a_map = vec![vec![false; n]; n];
    let mut b_map = vec![vec![false; n]; n];
    for (a, b) in takahashi {
        let (a, b) = (a - 1, b - 1);
        a_map[a][b] = true;
        a_map[b][a] = true;
    }
    for (c, d) in aoki {
        let (c, d) = (c - 1, d - 1);
        b_map[c][d] = true;
        b_map[d][c] = true;
    }
    let mut ans = false;
    for p in (0..n).permutations(n) {
        let mut is_same = true;
        for i in 0..n {
            for j in 0..n {
                if a_map[i][j] != b_map[p[i]][p[j]] {
                    is_same = false;
                }
            }
        }
        if is_same {
            ans = true;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
