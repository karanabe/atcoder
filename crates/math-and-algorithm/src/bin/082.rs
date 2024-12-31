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

    let mut ans = 0;

    while !lr.is_empty() {
        let min_r = lr.iter().min_by_key(|&(_,r)| r).unwrap().1;

        lr.retain(|&(l, _)| min_r <= l);

        ans += 1;
    }

    println!("{ans}");
}
