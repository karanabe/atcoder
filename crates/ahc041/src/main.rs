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
use rand::{
    rngs::StdRng,
    Rng,
    thread_rng,
    SeedableRng
};
#[allow(unused_imports)]
use rand_distr::{Normal, Distribution};
#[allow(unused_imports)]
use std::time::{Instant, Duration};


#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: usize,
        a: [usize; n],
        edges: [(usize, usize); m],
        coords: [(usize, usize); n],
    }

    let mut graph = vec![Vec::new(); n];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut parent = vec![-2isize; n];

    for v in 0..n {
        if parent[v] == -2 {
            parent[v] = -1;
            // dfs_assign(v, &graph, &mut parent, h);
        }
    }

    for i in 0..n {
        print!("{} ", parent[i]);
    }
    println!();
}
