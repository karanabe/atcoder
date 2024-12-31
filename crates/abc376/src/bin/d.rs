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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut graph = vec![Vec::new(); n + 1];
    let mut rev_graph = vec![Vec::new(); n + 1];

    for (a, b) in &edges {
        graph[*a].push(*b);
        rev_graph[*b].push(*a);
    }

    let mut visited = vec![false; n + 1];
    let mut order = Vec::new();
    for i in 1..=n {
        if !visited[i] {
            dfs1(i, &graph, &mut visited, &mut order);
        }
    }

    let mut visited = vec![false; n + 1];
    let mut scc_id = vec![0; n + 1];
    let mut current_scc = 0;
    for &u in order.iter().rev() {
        if !visited[u] {
            current_scc += 1;
            dfs2(u, &rev_graph, &mut visited, current_scc, &mut scc_id);
        }
    }

    let scc_of_one = scc_id[1];

    if scc_id.iter().filter(|&&id| id == scc_of_one).count() == 1 {
        println!("-1");
        return;
    }

    let mut scc_graph = vec![Vec::new(); n + 1];
    for &(a, b) in &edges {
        if scc_id[a] == scc_of_one && scc_id[b] == scc_of_one {
            scc_graph[a].push(b);
        }
    }

    let start = 1;
    let mut queue = VecDeque::new();
    let mut level = vec![-1; n + 1];
    let mut parent = vec![0; n + 1];
    let mut min_cycle = std::i32::MAX;

    queue.push_back(start);
    level[start] = 0;

    while let Some(u) = queue.pop_front() {
        for &v in &scc_graph[u] {
            if level[v] == -1 {
                level[v] = level[u] + 1;
                parent[v] = u;
                queue.push_back(v);
            } else if parent[u] != v {
                if u == 1 || v == 1 {
                    let cycle_length = level[u] - level[v] + 1;
                    if cycle_length >= 2 {
                        min_cycle = min_cycle.min(cycle_length);
                    }
                }
            }
        }
    }

    if min_cycle == std::i32::MAX {
        println!("-1");
    } else {
        println!("{}", min_cycle);
    }
}

fn dfs1(u: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[u] = true;
    for &v in &graph[u] {
        if !visited[v] {
            dfs1(v, graph, visited, order);
        }
    }
    order.push(u);
}

fn dfs2(u: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, label: usize, scc_id: &mut Vec<usize>) {
    visited[u] = true;
    scc_id[u] = label;
    for &v in &graph[u] {
        if !visited[v] {
            dfs2(v, graph, visited, label, scc_id);
        }
    }
}
