#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{Ordering, max, min};

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
    Max,
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&mut self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree,
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    math,
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

fn is_target(s: &[Vec<char>], i: usize, j: usize) -> bool {
    // ###.
    // ###.
    // ###.
    // ....
    for y in 0..4 {
        for x in 0..4 {
            let c = s[i + y][j + x];
            let ok = if y <= 2 && x <= 2 { c == '#' } else { c == '.' };
            if !ok {
                return false;
            }
        }
    }

    // ....
    // .###
    // .###
    // .###
    for y in 0..4 {
        for x in 0..4 {
            let c = s[i + 5 + y][j + 5 + x];
            let ok = if y >= 1 && x >= 1 { c == '#' } else { c == '.' };
            if !ok {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    for i in 0..=n.saturating_sub(9) {
        for j in 0..=m.saturating_sub(9) {
            if is_target(&s, i, j) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
