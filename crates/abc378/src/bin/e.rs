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
    Segtree,
    Max
};


fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
    }

    let mut prefix = vec![0u64; n + 1];
    for i in 0..n {
        prefix[i + 1] = (prefix[i] + a[i]) % m as u64;
    }

    let mut sum_r_p: u64 = 0;
    for r in 1..=n {
        sum_r_p = sum_r_p.wrapping_add((r as u64) * prefix[r]);
    }

    let mut sum_p_times: u64 = 0;
    for l in 0..n {
        sum_p_times = sum_p_times.wrapping_add(prefix[l] * ((n - l) as u64));
    }

    let sum1 = sum_r_p.wrapping_sub(sum_p_times);

    let mut fenwick = FenwickTree::new(m + 1, 0u64);
    let mut sum2: u64 = 0;

    fenwick.add(prefix[0] as usize, 1u64);

    for r in 1..=n {
        let pr = prefix[r] as usize;
        if pr < m {
            let count = fenwick.sum(pr + 1..=m);
            sum2 = sum2.wrapping_add(count);
        }
        fenwick.add(pr, 1u64);
    }

    let ans = sum1.wrapping_add((m as u64).wrapping_mul(sum2));

    println!("{ans}");
}
