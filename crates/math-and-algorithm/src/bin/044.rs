#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn bfs(n: usize, graph: &Vec<Vec<usize>>, start: usize) -> Vec<isize> {
    let mut distance = vec![-1; n];
    distance[start] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &next in &graph[node] {
            if distance[next] == -1 {
                distance[next] = distance[node] + 1;
                queue.push_back(next);
            }
        }
    }

    distance
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let distances = bfs(n, &graph, 0);

    for dist in distances {
        println!("{}", dist);
    }
}
