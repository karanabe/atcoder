use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            u: Usize1,
            v: Usize1,
        }

        graph[u].push(v);
        graph[v].push(u);
    }

    input! {
        actives: [Usize1; k],
    }

    let mut is_active = vec![false; n];
    for &v in &actives {
        is_active[v] = true;
    }

    let result = dfs(actives[0], None, &graph, &is_active);

    println!("{}", result);
}

fn dfs(v: usize, p: Option<usize>, graph: &Vec<Vec<usize>>, is_active: &Vec<bool>) -> usize {
    let mut subtree_size = 0;

    for &nv in &graph[v] {
        if Some(nv) == p {
            continue;
        }

        subtree_size += dfs(nv, Some(v), graph, is_active);
    }

    if is_active[v] || subtree_size > 0 {
        subtree_size += 1;
    }

    subtree_size
}
