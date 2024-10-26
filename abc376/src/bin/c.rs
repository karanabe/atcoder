#[allow(unused_imports)]
use proconio::{
    input,
    fastout,
    marker::{Isize1,Usize1,Chars,Bytes}
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{
    VecDeque,
    LinkedList,
    HashMap,
    BTreeMap,
    HashSet,
    BTreeSet,
    BinaryHeap
};

#[allow(unused_imports)]
use std::cmp::{
    min,
    max,
    Ordering
};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        mut b: [u32; n - 1],
    }

    a.sort();
    b.sort();

    let max_x = 2_000_000_010;
    let mut left = 1;
    let mut right = max_x;

    let check = |x: u32| -> bool {
        let pos = match b.binary_search(&x) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        for i in 0..n {
            let box_size = if i < pos {
                b[i]
            } else if i == pos {
                x
            } else {
                b[i - 1]
            };

            if a[i] > box_size {
                return false;
            }
        }
        true
    };

    let mut ans = -1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if check(mid as u32) {
            ans = mid as i64;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    println!("{}", if ans != -1 { ans } else { -1 });
}
