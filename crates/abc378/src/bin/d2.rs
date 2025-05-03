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

#[allow(unused_variables)]
const DIRECTION: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut ans: usize = 0;
    let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                dfs(&s, &mut seen, (i, j), h, w, k, &mut ans, 0);
            }
        }
    }
    println!("{ans}");
}

fn dfs(
    grid: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    pos: (usize, usize),
    h: usize,
    w: usize,
    k: usize,
    ans: &mut usize,
    count: usize,
) -> bool {
    if seen[pos.0][pos.1] {
        return false;
    }

    if count == k {
        *ans += 1;
        return true;
    }

    seen[pos.0][pos.1] = true;

    for (i, j) in DIRECTION {
        let next = (pos.0.wrapping_add(i), pos.1.wrapping_add(j));
        if next.0 < h && next.1 < w && (grid[next.0][next.1] == '.') {
            dfs(grid, seen, next, h, w, k, ans, count + 1);
        }
    }

    seen[pos.0][pos.1] = false;

    false
}
