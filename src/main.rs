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

fn dfs(cur: usize, links: &[Vec<usize>], dst: &mut [i32], d: i32) {
    // 終了条件
    if dst[cur] != -1 {
        return;
    }
    dst[cur] = if d % 2 == 0 { 1 } else { 0 };
    let d = d + 1;
    for &next_idx in &links[cur] {
        dfs(next_idx, links, dst, d)
    }
}

fn main() {
    input! {n:usize,q:usize,roads:[(usize,usize);n-1],queries:[(usize,usize);q]}
    let mut links = vec![vec![]; n];
    for (a, b) in roads {
        let a = a - 1;
        let b = b - 1;
        links[a].push(b);
        links[b].push(a)
    }

    let mut is_even = vec![-1; n];
    dfs(0, &links, &mut is_even, 0);
    for (c, d) in queries {
        let c = c - 1;
        let d = d - 1;
        if is_even[c] == is_even[d] {
            println!("Town")
        } else {
            println!("Road")
        }
    }
}
