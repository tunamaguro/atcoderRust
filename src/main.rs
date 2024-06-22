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
    input! {n:usize,a:[i64;n]}
    let mut ans = 0;
    let mut cur_pos = 0;
    let mut cur_advance = 0;
    let mut cur_max_advance = 0;
    for ai in a {
        cur_max_advance = cur_max_advance.max(cur_advance + ai);
        cur_advance += ai;

        // dbg!(cur_pos, cur_advance, cur_max_advance);

        ans = ans.max(cur_pos + cur_max_advance);
        cur_pos += cur_advance;
    }
    println!("{}", ans);
}
