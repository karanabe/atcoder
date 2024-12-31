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
    max
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


fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n - 1],
    }

    let mut adj = vec![Vec::new(); n + 1];
    let mut degs = vec![0; n + 1];

    for &(u, v) in &edges {
        adj[u].push(v);
        adj[v].push(u);
        degs[u] += 1;
        degs[v] += 1;
    }

    let mut deg3_nodes = Vec::new();
    for i in 1..=n {
        if degs[i] == 3 {
            deg3_nodes.push(i);
        }
    }

    let mut deg3_adj = vec![Vec::new(); n + 1];
    for &node in &deg3_nodes {
        for &next in &adj[node] {
            if degs[next] == 3 {
                deg3_adj[node].push(next);
            }
        }
    }

    let mut visited = vec![false; n + 1];
    let mut ttl_cyc = 0;

    for &node in &deg3_nodes {
        if !visited[node] {
            let mut stack = VecDeque::new();
            stack.push_back(node);
            visited[node] = true;

            let mut component_nodes = Vec::new();

            while let Some(current) = stack.pop_front() {
                component_nodes.push(current);
                for &next in &deg3_adj[current] {
                    if !visited[next] {
                        visited[next] = true;
                        stack.push_back(next);
                    }
                }
            }

            let mut deg2_set = HashSet::new();
            for &comp_node in &component_nodes {
                for &next in &adj[comp_node] {
                    if degs[next] == 2 {
                        deg2_set.insert(next);
                    }
                }
            }

            let k = deg2_set.len();
            if k >= 2 {
                ttl_cyc += k * (k - 1) / 2;
            }
        }
    }

    println!("{}", ttl_cyc);
}
