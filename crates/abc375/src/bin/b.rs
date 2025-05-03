#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut ans = 0.0;

    let mut base = (0.0, 0.0);

    for (x, y) in xy {
        ans += ((base.0 - x).powf(2.0) + (base.1 - y).powf(2.0)).sqrt();
        base = (x, y);
    }
    ans += ((base.0 - 0.0).powf(2.0) + (base.1 - 0.0).powf(2.0)).sqrt();

    println!("{ans}");
}
