use proconio::{input, marker::Chars};

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

trait Rotate {
    fn rotate90(&self) -> Self;
}
impl<T: Copy + Default> Rotate for Vec<Vec<T>> {
    fn rotate90(&self) -> Self {
        let row = self.len();
        let col = self.get(0).unwrap().len();
        let mut result = vec![vec![T::default(); col]; row];

        for i in 0..row {
            for j in 0..col {
                let ii = row - i - 1;
                result[j][ii] = self[i][j];
            }
        }

        result
    }
}

fn main() {
    input! {n:usize,s:[Chars;n],t_orig:[Chars;n]}
    let mut ans = false;
    for rotate in 0..4 {
        // println!("-------------------------------");
        let mut t = t_orig.clone();
        for _ in 0..rotate {
            t = t.rotate90();
        }

        let mut s_top_left = (n, n);
        let mut t_top_left = (n, n);
        for i in 0..n {
            for j in 0..n {
                if s[i][j] == '#' {
                    s_top_left = (s_top_left.0.min(i), s_top_left.1.min(j));
                }
                if t[i][j] == '#' {
                    t_top_left = (t_top_left.0.min(i), t_top_left.1.min(j));
                }
                // print!("{}", t[i][j]);
            }
            // println!();
        }
        // ずらした後の位置
        let mut ss = vec![vec![false; n]; n];
        let mut tt = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if s[i][j] == '#' {
                    ss[i - s_top_left.0][j - s_top_left.1] = true;
                }
                if t[i][j] == '#' {
                    tt[i - t_top_left.0][j - t_top_left.1] = true;
                }
            }
        }

        ans |= ss == tt;
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
