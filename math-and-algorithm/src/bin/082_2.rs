#[allow(unused_imports)]
use proconio::{
    input,
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

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by_key(|&(_, r)| r);
    let mut ans = 0;
    let mut current_time = 0;

    for &(l, r) in &lr {
        if l >= current_time {
            ans += 1;
            current_time = r;
        }
    }

    println!("{ans}");
}
