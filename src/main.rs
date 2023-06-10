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
    input! {n:usize,a:[usize;n],q:usize,query:[(usize,usize);q]}
    let mut memo = vec![0];
    for (i, v) in a.iter().enumerate().skip(1) {
        let last = memo.last().unwrap();
        if i % 2 == 0 {
            let sleep = v - a[i - 1];
            memo.push(last + sleep)
        } else {
            memo.push(*last)
        }
    }
    // println!("{:?}", memo);
    for (l, r) in query {
        let left = a.lower_bound(&l);
        let right = a.lower_bound(&r);
        // println!("l: {}  r: {}", l, r);
        // println!(
        //     "left: {} ({}) right: {} ({})",
        //     left, a[left], right, a[right]
        // );

        let mut ans = memo[right] - memo[left];
        if left % 2 == 0 {
            // println!("left tyousei");
            ans += a[left] - l
        }
        if right % 2 == 0 {
            // println!("right tyousei");
            ans -= a[right] - r;
        }

        println!("{}", ans)
    }
}
