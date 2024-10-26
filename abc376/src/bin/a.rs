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
    max
};

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
