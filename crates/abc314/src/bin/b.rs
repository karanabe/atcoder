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
        n: usize
    }

    let mut bet = Vec::<Vec<usize>>::with_capacity(n);

    for _ in 0..n {
        input! {
            c: usize,
            a: [usize; c]
        }
        bet.push(a);
    }

    input! {
        x: usize
    }

    let mut count: HashMap<usize, usize> = HashMap::new();
    for (i, _) in bet.iter().enumerate().take(n) {
        if bet[i].contains(&x) {
            count.insert(i + 1, bet[i].len());
        }
    }

    let Some(min_count) = count.values().copied().min() else {
        println!("0");
        println!();
        return;
    };

    let mut keys: Vec<usize> = count
        .iter()
        .filter_map(|(&k, &v)| (v == min_count).then_some(k))
        .collect();

    keys.sort_unstable();

    println!("{}", keys.len());
    println!("{}", keys.iter().join(" "));
}
