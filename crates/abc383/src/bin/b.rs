#[allow(unused_imports)]
use proconio::{
    input,
    input_interactive,
    fastout,
    source::line::LineSource,
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

#[allow(unused_imports)]
use std::io::{stdout, Write};

#[allow(unused_imports)]
use rand::{Rng, thread_rng};
#[allow(unused_imports)]
use rand_distr::{Normal, Distribution};

#[allow(dead_code)]
const DIRECTION: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        grid: [Chars; h],
    }

    let mut cell = Vec::new();
    for i in 0..h {
        let row = &grid[i];
        for j in 0..w {
            if row[j] == '.' {
                cell.push((i,j));
            }
        }
    }
    let f = cell.len();

    let mut coverage_cell = vec![0u128; f];
    for (i, &(x, y)) in cell.iter().enumerate() {
        let mut mask = 0u128;
        for (f_idx, &(fx, fy)) in cell.iter().enumerate() {
            let dist = (x as isize - fx as isize).abs() + (y as isize - fy as isize).abs();
            if dist as usize <= d {
                mask |= 1 << f_idx;
            }
        }
        coverage_cell[i] = mask;
    }

    let mut ans = 0;
    for i in 0..f {
        for j in i+1..f {
            let u_mask = coverage_cell[i] | coverage_cell[j];
            ans = ans.max(u_mask.count_ones());
        }
    }

    println!("{}", ans);
}
