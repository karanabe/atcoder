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
    // accum(&self, index: usize) -> T
    // add<U: Clone>(&mut self, index: usize, value: U)
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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut map: Vec<(usize, usize)> = a.into_iter().enumerate().map(|(i, v)| (v, i + 1)).collect();

    map.sort_unstable_by_key(|&(v, _)| v);

    let mut min_result = vec![0; map.len()];
    let mut min_index = map[0].1;
    min_result[0] = min_index;

    for i in 1..map.len() {
        if map[i].1 < min_index {
            min_index = map[i].1;
        }
        min_result[i] = min_index;
    }

    for oishisa in b {
        let index = map.binary_search_by(|&(value, _)| {
            if value <= oishisa {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        let pos = match index {
            Ok(pos) => {
                let mut last_pos = pos;
                while last_pos + 1 < map.len() && map[last_pos + 1].0 == map[pos].0 {
                    last_pos += 1;
                }
                last_pos
            }
            Err(pos) => {
                if pos == 0 {
                    println!("-1");
                    continue;
                } else {
                    pos - 1
                }
            }
        };

        println!("{}", min_result[pos]);
    }
}
