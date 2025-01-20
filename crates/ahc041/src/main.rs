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

#[allow(dead_code)]
fn dfs(
    root: usize,
    graph: &Vec<Vec<usize>>,
    a: &Vec<usize>,
    used_global: &Vec<bool>,
    h: usize
) -> (Vec<isize>, usize, Vec<usize>) {
    let n = graph.len();

    let mut parent = vec![-1isize; n];
    let mut dist   = vec![-1isize; n];
    let mut score: usize = 0;
    let mut used_vertices = vec![];

    dist[root] = 0;
    parent[root] = -1;

    let mut stack = Vec::new();
    stack.push(root);

    while let Some(u) = stack.pop() {
        let du = dist[u];
        score += (h + 1) * a[u];
        used_vertices.push(u);

        if du < h as isize {
            for &v in &graph[u] {
                if !used_global[v] && dist[v] == -1 {
                    dist[v] = du + 1;
                    parent[v] = u as isize;
                    stack.push(v);
                }
            }
        }
    }

    (parent, score, used_vertices)
}

fn dfs_recursive(
    u: usize,
    graph: &Vec<Vec<usize>>,
    a: &Vec<usize>,
    used_global: &Vec<bool>,
    parent: &mut Vec<isize>,
    dist: &mut Vec<isize>,
    h: usize,
    score: &mut usize,
    used_vertices: &mut Vec<usize>,
) {
    *score += (h + 1) * a[u];
    used_vertices.push(u);

    let du = dist[u];
    if du < h as isize {
        for &v in &graph[u] {
            if !used_global[v] && dist[v] == -1 {
                dist[v] = du + 1;
                parent[v] = u as isize;
                dfs_recursive(v, graph, a, used_global, parent, dist, h, score, used_vertices);
            }
        }
    }
}

fn dfs_wrapper(
    root: usize,
    graph: &Vec<Vec<usize>>,
    a: &Vec<usize>,
    used_global: &Vec<bool>,
    h: usize
) -> (Vec<isize>, usize, Vec<usize>) {
    let n = graph.len();
    let mut parent = vec![-1isize; n];
    let mut dist   = vec![-1isize; n];
    let mut score: usize = 0;
    let mut used_vertices = vec![];

    dist[root] = 0;
    parent[root] = -1;

    if !used_global[root] {
        dfs_recursive(
            root,
            graph,
            a,
            used_global,
            &mut parent,
            &mut dist,
            h,
            &mut score,
            &mut used_vertices
        );
    }

    (parent, score, used_vertices)
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

    let mut used_global = vec![false; n];
    let mut parent = vec![-2isize; n];

    loop {
        let mut best_root = None;
        let mut best_score = 0;
        let mut best_parent = vec![-1isize; n];
        let mut best_vertices = vec![];

        for v in 0..n {
            if used_global[v] {
                continue;
            }

            // Find best tree with BFS
            let (parent_tmp, score_tmp, used_tmp) =
                dfs_wrapper(v, &graph, &a, &used_global, h);

            if score_tmp > best_score {
                best_score = score_tmp;
                best_root = Some(v);
                best_parent = parent_tmp;
                best_vertices = used_tmp;
            }
        }

        // If we can not create tree any more
        if best_score == 0 || best_root.is_none() {
            break;
        }

        // Update vertices to the best score
        for &u in &best_vertices {
            used_global[u] = true;
        }

        // Update to result array
        for i in 0..n {
            if best_parent[i] >= 0 {
                parent[i] = best_parent[i];
            } else if best_parent[i] == -1 {
                if i == best_root.unwrap() {
                    parent[i] = -1;
                }
            }
        }
    }

    for i in 0..n {
        print!("{} ", parent[i]);
    }
    println!();
}
