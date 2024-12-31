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

#[allow(unused_imports)]
use ac_library::{
    Dsu,
    // new(size: usize) -> Self
    // merge(&mut self, a: usize, b: usize) -> usize
    // same(&mut self, a: usize, b: usize) -> bool
    // leader(&mut self, a: usize) -> usize
    // size(&mut self, a: usize) -> usize
    // groups(&mut self) -> Vec<Vec<usize>>
    FenwickTree,
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    math,
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&mut self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree
};

fn count_pairs(n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;
    let mut bit = FenwickTree::new(n + 2, 0);

    for i in (1..=n).rev() {
        let ai = a[i - 1];
        if ai == i {
            ans += bit.sum((i + 1)..(n + 2));
        }
        if ai == i {
            bit.add(i, 1);
        }
    }

    for i in 1..=n {
        let ai = a[i - 1];
        if ai > i && a[ai - 1] == i {
            ans += 1;
        }
    }

    ans
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let ans = count_pairs(n, a);
    println!("{ans}");
}
