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
        s: String,
        q: usize,
        ks: [u64; q],
    }

    let s_bytes = s.as_bytes();
    let len_s = s_bytes.len() as u128;

    let mut len_s_n = Vec::with_capacity(61);
    len_s_n.push(len_s);
    for n in 1..=60 {
        len_s_n.push(len_s_n[n - 1] * 2);
    }

    for &k in &ks {
        let mut ki = k as u128;
        let mut flip = false;
        let mut n = 0;

        while n < 60 && len_s_n[n] < ki {
            n += 1;
        }

        while n > 0 {
            let len_half = len_s_n[n - 1];
            if ki > len_half {
                ki -= len_half;
                flip = !flip;
            }
            n -= 1;
        }

        if ki == 0 || ki > len_s {
            println!(" ");
            continue;
        }

        let mut ans = s_bytes[(ki - 1) as usize] as char;
        if flip {
            if ans.is_lowercase() {
                ans = ans.to_uppercase().next().unwrap();
            } else {
                ans = ans.to_lowercase().next().unwrap();
            }
        }
        print!("{} ", ans);
    }
    println!();
}
