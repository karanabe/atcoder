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

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        grid: [Chars; h],
    }

    let hw = h * w;
    let mut dist = vec![u32::MAX; hw];
    let mut q = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'H' {
                let idx = i * w + j;
                dist[idx] = 0;
                q.push_back(idx);
            }
        }
    }

    let directions = [(1isize,0isize),(-1,0),(0,1),(0,-1)];
    while let Some(idx) = q.pop_front() {
        let dii = dist[idx];
        if dii < d as u32 {
            let x = idx / w;
            let y = idx % w;
            for &(dx,dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize { continue; }
                let nxu = nx as usize;
                let nyu = ny as usize;
                if grid[nxu][nyu] == '#' { continue; }
                let nidx = nxu * w + nyu;
                let nd = dii + 1;
                if dist[nidx] > nd {
                    dist[nidx] = nd;
                    q.push_back(nidx);
                }
            }
        }
    }

    let mut count = 0u32;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != '#' && dist[i*w+j] <= d as u32 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

