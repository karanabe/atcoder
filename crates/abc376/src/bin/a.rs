#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        t: [usize; n],
    }

    let mut base_time = t[0];
    let mut ans = 1;

    for curr_time in t {
        if c <= curr_time - base_time {
            base_time = curr_time;
            ans += 1;
        }
    }

    println!("{ans}");
}
