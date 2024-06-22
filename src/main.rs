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
    input! {mut sx:i64,sy:i64,mut tx:i64,ty:i64}
    // xをパネルの左側にいることにする
    if sy % 2 == 0 {
        sx -= sx % 2;
    } else {
        sx -= (sx + 1) % 2;
    }
    if ty % 2 == 0 {
        tx -= tx % 2;
    } else {
        tx -= (tx + 1) % 2;
    }

    // dbg!(sx, sy, tx, ty);

    let dx = tx - sx;
    let dy = (ty - sy).abs();

    // 上に上がる際の横移動で足りる
    if dx.abs() <= dy {
        println!("{}", dy);
        return;
    }
    let dx = (dx.abs() - dy) * dx.signum();
    // dbg!(dx);
    let ans = dy + dx.abs() / 2;
    println!("{}", ans)
}
