#[allow(unused_imports)]
use proconio::{
    fastout, input, input_interactive,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::line::LineSource,
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};

#[allow(unused_imports)]
use ac_library::{
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    math,
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
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

#[allow(unused_imports)]
use std::io::{stdout, Write};

#[allow(unused_imports)]
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
#[allow(unused_imports)]
use rand_distr::{Distribution, Normal};
#[allow(unused_imports)]
use std::time::{Duration, Instant};

fn dfs(
    root: usize,
    graph: &Vec<Vec<usize>>,
    parent: &mut Vec<isize>,
    height: &mut Vec<usize>,
    a: &Vec<usize>,
    h: usize,
) {
    let current_height = height[root];
    if current_height < h {
        for &v in &graph[root] {
            if parent[v] == -2 {
                height[v] = current_height + 1;
                parent[v] = root as isize;
                dfs(v, graph, parent, height, &a, h);
            }
        }
    }
}

#[allow(dead_code)]
fn bfs(
    root: usize,
    graph: &Vec<Vec<usize>>,
    parent: &mut Vec<isize>,
    a: &Vec<usize>,
    h: usize,
    n: usize,
) {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    // graph height
    let mut height = vec![0usize; n];
    height[root] = 0;

    parent[root] = -1;

    while let Some(vertices) = queue.pop_front() {
        let current_height = height[vertices];

        if current_height < h {
            for &v in &graph[vertices] {
                // Check by parent or height array
                if parent[v] == -2 {
                    height[v] = current_height + 1;
                    parent[v] = vertices as isize;
                    queue.push_back(v);
                }
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        h: usize,
        a: [usize; n],
        edges: [(usize, usize); m],
        _coords: [(usize, usize); n],
    }

    let mut graph = vec![Vec::new(); n];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    // sort dfs order by asc
    for u in 0..n {
        graph[u].sort_by_key(|&v| a[v]);
    }

    let mut parent = vec![-2isize; n];
    let mut height = vec![0usize; n];

    for v in 0..n {
        if parent[v] == -2 {
            // bfs(v, &graph, &mut parent, &a, h, n);
            height[v] = 0;
            parent[v] = -1;
            dfs(v, &graph, &mut parent, &mut height, &a, h);
        }
    }

    for i in 0..n {
        print!("{} ", parent[i]);
    }
    println!();
}
