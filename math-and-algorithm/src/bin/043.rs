#[allow(unused_imports)]
use proconio::{
    input,
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

#[allow(dead_code)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[allow(dead_code)]
fn dfs(node: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[node] = true;
    for &next in &graph[node] {
        if !visited[next] {
            dfs(next, graph, visited);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m]
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n];

    dfs(0, &graph, &mut visited);

    if visited.iter().all(|&v| v) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

/*
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);

    for (a, b) in edges {
        uf.union(a, b);
    }

    let root = uf.find(0);
    let connected = (1..n).all(|i| uf.find(i) == root);

    if connected {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}
*/
