use proconio::input;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    top_vertices: Vec<Vec<usize>>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut top_vertices = Vec::with_capacity(n);
        for i in 0..n {
            top_vertices.push(vec![i]);
        }
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            top_vertices,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let orig_parent = self.parent[x];
            self.parent[x] = self.find(orig_parent);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];

        let merged_top = Self::merge_top_vertices(
            &self.top_vertices[root_x],
            &self.top_vertices[root_y],
        );
        self.top_vertices[root_x] = merged_top;
    }

    fn merge_top_vertices(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
        let mut result = Vec::with_capacity(20);
        let mut i = 0;
        let mut j = 0;
        while result.len() < 10 {
            if i < a.len() && (j >= b.len() || a[i] > b[j]) {
                result.push(a[i]);
                i += 1;
            } else if j < b.len() {
                result.push(b[j]);
                j += 1;
            } else {
                break;
            }
        }
        result
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n);
    let mut results = Vec::new();

    for _ in 0..q {
        input! {
            t: u8,
        }

        if t == 1 {
            input! {
                u: usize,
                v: usize,
            }
            let u = u - 1;
            let v = v - 1;
            uf.union(u, v);
        } else {
            input! {
                v: usize,
                k: usize,
            }
            let v = v - 1;
            let root_v = uf.find(v);
            let top_vertices = &uf.top_vertices[root_v];
            if k <= top_vertices.len() {
                let ans = top_vertices[k - 1] + 1;
                results.push(ans.to_string());
            } else {
                results.push("-1".to_string());
            }
        }
    }

    for ans in results {
        println!("{}", ans);
    }
}
