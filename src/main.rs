#![allow(dead_code)]
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
    input! {a:i32,b:i32,c:i32,d:i32}
    let mut arr = [a, b, c, d];
    arr.sort();

    let have_three =
        (arr[0] == arr[1] && arr[1] == arr[2]) || (arr[1] == arr[2] && arr[2] == arr[3]);

    let have_double_two = (arr[0] == arr[1]) && arr[2] == arr[3];

    let all_same = arr[0] == arr[1] && arr[1] == arr[2] && arr[2] == arr[3];

    if !all_same && (have_three || have_double_two) {
        println!("Yes")
    } else {
        println!("No")
    }
}
