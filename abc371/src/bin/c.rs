use proconio::input;
use proconio::marker::Usize1;
use itertools::Itertools;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
    }

    input! {
        mg: usize,
    }
    let mut g_adj = vec![vec![false; n]; n];
    for _ in 0..mg {
        input! {
            u: Usize1,
            v: Usize1,
        }
        g_adj[u][v] = true;
        g_adj[v][u] = true;
    }

    input! {
        mh: usize,
    }
    let mut h_adj = vec![vec![false; n]; n];
    for _ in 0..mh {
        input! {
            a: Usize1,
            b: Usize1,
        }
        h_adj[a][b] = true;
        h_adj[b][a] = true;
    }

    let mut aij = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {
            costs: [usize; n - i - 1],
        }
        for (j, &cost) in costs.iter().enumerate() {
            aij[i][i + j + 1] = cost;
            aij[i + j + 1][i] = cost;
        }
    }

    let mut min_cost = std::usize::MAX;

    for perm in (0..n).permutations(n) {
        let mut t_adj = vec![vec![false; n]; n];
        for i in 0..n {
            for j in i + 1..n {
                t_adj[i][j] = g_adj[perm[i]][perm[j]];
                t_adj[j][i] = t_adj[i][j];
            }
        }

        let mut cost = 0;
        for i in 0..n {
            for j in i + 1..n {
                if h_adj[i][j] != t_adj[i][j] {
                    cost += aij[i][j];
                }
            }
        }

        min_cost = min(min_cost, cost);
    }

    println!("{}", min_cost);
}
