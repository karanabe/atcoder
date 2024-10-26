#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Isize1,Usize1,Chars,Bytes}
};
#[allow(unused_imports)]
use itertools;

struct UnionFind {
    parent: Vec<usize>,
    potential: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            potential: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let par = self.parent[x];
            let r = self.find(par);
            self.potential[x] += self.potential[par];
            self.parent[x] = r;
        }
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize, w: i64) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }

        self.parent[rx] = ry;
        self.potential[rx] = self.potential[y] - self.potential[x] - w;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, i64); m],
    }

    let mut uf = UnionFind::new(n);

    for &(u, v, w) in &edges {
        let u_idx = u - 1;
        let v_idx = v - 1;
        uf.unite(u_idx, v_idx, w);
    }

    for i in 0..n {
        uf.find(i);
    }

    let mut result = vec![0i64; n];
    for i in 0..n {
        result[i] = uf.potential[i];
    }

    for (i, ans) in result.iter().enumerate() {
        if i == n - 1 {
            println!("{}", ans);
        } else {
            print!("{} ", ans);
        }
    }
}
