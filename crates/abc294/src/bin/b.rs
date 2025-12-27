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

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut out = String::new();
    for row in a.iter() {
        for &x in row.iter() {
            out.push(cell_to_char(x));
        }
        out.push('\n');
    }
    print!("{out}");
}

fn cell_to_char(x: usize) -> char {
    if x == 0 {
        '.'
    } else {
        (b'A' + (x - 1) as u8) as char
    }
}
