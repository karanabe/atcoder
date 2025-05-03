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
        mut lr: [(usize, usize); n],
    }

    let mut ans = 0;

    while !lr.is_empty() {
        let min_r = lr.iter().min_by_key(|&(_, r)| r).unwrap().1;

        lr.retain(|&(l, _)| min_r <= l);

        ans += 1;
    }

    println!("{ans}");
}
