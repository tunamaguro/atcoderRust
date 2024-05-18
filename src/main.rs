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
    input! {a:i64,b:i64,c:i64,d:i64}

    let h = d - b;
    let w = c - a;
    let area = [(2, 1), (1, 2), (0, 1), (1, 0)];
    let mut cur_x = a;
    let mut ans = 0;

    let left_idx = (a + 10_i64.pow(9)) % 4;
    let is_odd = b % 2 == 0;

    let count = (w + 3) / 4;
    let tmp = area[left_idx as usize];
    let mut col_size = 0;
    if is_odd {
        col_size += tmp.0 * ((h + 1) / 2); // １段目
        col_size += tmp.1 * (h / 2); // 2段目
    } else {
        col_size += tmp.1 * ((h + 1) / 2); // １段目
        col_size += tmp.0 * (h / 2); // 2段目
    }
    ans += col_size * count;

    let next_idx = (left_idx + 1) % 4;
    let count = (w + 2) / 4;
    let tmp = area[next_idx as usize];
    let mut col_size = 0;
    if is_odd {
        col_size += tmp.0 * ((h + 1) / 2); // １段目
        col_size += tmp.1 * (h / 2); // 2段目
    } else {
        col_size += tmp.1 * ((h + 1) / 2); // １段目
        col_size += tmp.0 * (h / 2); // 2段目
    }
    ans += col_size * count;

    let next_idx = (next_idx + 1) % 4;
    let count = (w + 1) / 4;
    let tmp = area[next_idx as usize];
    let mut col_size = 0;
    if is_odd {
        col_size += tmp.0 * ((h + 1) / 2); // １段目
        col_size += tmp.1 * (h / 2); // 2段目
    } else {
        col_size += tmp.1 * ((h + 1) / 2); // １段目
        col_size += tmp.0 * (h / 2); // 2段目
    }
    ans += col_size * count;

    let next_idx = (next_idx + 1) % 4;
    let count = (w) / 4;
    let tmp = area[next_idx as usize];
    let mut col_size = 0;
    if is_odd {
        col_size += tmp.0 * ((h + 1) / 2); // １段目
        col_size += tmp.1 * (h / 2); // 2段目
    } else {
        col_size += tmp.1 * ((h + 1) / 2); // １段目
        col_size += tmp.0 * (h / 2); // 2段目
    }
    ans += col_size * count;

    println!("{}", ans)
}
