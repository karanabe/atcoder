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
    Segtree,
    Max
};

fn main() {
    input! {
        n: i64,
        m: usize,
        x: [i64; m],
        a: [i64; m],
    }

    let mut xa: Vec<(i64, i64)> = x.into_iter().zip(a.into_iter()).collect();
    xa.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum = 0i64;
    let mut sum_idx = 0i64;

    for i in 0..m {
        let pos = xa[i].0;
        let stones = xa[i].1;

        if sum < pos - 1 {
            println!("-1");
            return;
        }

        sum += stones;
        sum_idx += stones * pos;
    }

    if sum != n {
        println!("-1");
        return;
    }

    let ans = n * (n + 1) / 2 - sum_idx;
    println!("{}", ans);
}
