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
        k: usize,
        s: String,
    }

    let sch: Vec<char> = s.chars().collect();
    let mut block = Vec::new();
    let mut i = 0;

    while i < n {
        if sch[i] == '1' {
            let start = i;
            while i + 1 < n && sch[i + 1] == '1' {
                i += 1;
            }
            let end = i;
            block.push((start, end));
        }
        i += 1;
    }

    let (_, rk1) = block[k - 2];
    let (l_k, r_k) = block[k - 1];
    let len = r_k - l_k + 1;

    let mut tch = sch.clone();

    for i in l_k..=r_k {
        tch[i] = '0';
    }

    for i in (rk1 + 1)..=(rk1 + len) {
        if i < n {
            tch[i] = '1';
        }
    }

    for i in (rk1 + len + 1)..=r_k {
        if i < n {
            tch[i] = '0';
        }
    }

    let ans: String = tch.into_iter().collect();
    println!("{}", ans);
}
