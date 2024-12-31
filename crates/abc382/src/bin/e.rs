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

#[allow(unused_imports)]
use num::{
    BigInt,
    Zero
};


fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    }

    let mut p_i = Vec::with_capacity(n);
    for &pi in &p {
        p_i.push(pi as f64 / 100.0);
    }

    let mut p = vec![0.0; n + 1];
    p[0] = 1.0;
    let mut max = 0;
    for &pi in &p_i {
        let limit = min(max + 1, n);
        for s in (1..=limit).rev() {
            p[s] = p[s] * (1.0 - pi) + p[s - 1] * pi;
        }
        p[0] *= 1.0 - pi;
        max = min(max + 1, n);
    }

    let mut dp = vec![0.0; x + 1];

    let inv_p = 1.0 - p[0];

    for cx in (0..x).rev() {
        let limit = if x > cx + 1 { min(n, x - cx - 1) } else { 0 };
        let mut sum = 0.0;
        if limit > 0 {
            for s in 1..=limit {
                sum += p[s] * dp[cx + s];
            }
        }
        dp[cx] = (1.0 + sum) / inv_p;
    }

    println!("{:.16}", dp[0]);
}
