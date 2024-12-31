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

#[allow(unused_imports)]
use num::{
    BigInt,
    Zero
};

fn main() {
    input! {
        n: usize,
        fish_positions: [(usize, usize); 2 * n],
    }

    let grid_size = 1000;
    let cell_size = 100;
    let _max_coord = 100_000;

    let mut mackerel_grid = vec![vec![0usize; grid_size + 1]; grid_size + 1];
    let mut sardine_grid = vec![vec![0usize; grid_size + 1]; grid_size + 1];

    for (i, &(x, y)) in fish_positions.iter().enumerate() {
        let grid_x = x / cell_size;
        let grid_y = y / cell_size;
        if grid_x > grid_size || grid_y > grid_size {
            continue;
        }
        if i < n {
            mackerel_grid[grid_x][grid_y] += 1;
        } else {
            sardine_grid[grid_x][grid_y] += 1;
        }
    }

    let mut mackerel_prefix = vec![vec![0usize; grid_size + 2]; grid_size + 2];
    let mut sardine_prefix = vec![vec![0usize; grid_size + 2]; grid_size + 2];

    for i in 0..=grid_size {
        for j in 0..=grid_size {
            mackerel_prefix[i + 1][j + 1] = mackerel_grid[i][j]
                + mackerel_prefix[i][j + 1]
                + mackerel_prefix[i + 1][j]
                - mackerel_prefix[i][j];
            sardine_prefix[i + 1][j + 1] = sardine_grid[i][j]
                + sardine_prefix[i][j + 1]
                + sardine_prefix[i + 1][j]
                - sardine_prefix[i][j];
        }
    }

    let sizes = vec![10, 20, 30, 40, 50, 100, 200, 400, 500, 600, 700, 750, 800, 850, 900, 950, 1000];
    let step = 1;

    let mut best_score = std::i32::MIN;
    let mut best_rect = (0, 0, 0, 0);
    let perimeter_limit = 400_000;

    for &w in &sizes {
        for &h in &sizes {
            let perimeter = (w + h) * cell_size * 2;
            if perimeter > perimeter_limit {
                continue;
            }
            let max_x = grid_size - w;
            let max_y = grid_size - h;
            for x in (0..=max_x).step_by(step) {
                for y in (0..=max_y).step_by(step) {
                    let x1 = x;
                    let y1 = y;
                    let x2 = x + w;
                    let y2 = y + h;

                    let mackerels = mackerel_prefix[x2][y2]
                        - mackerel_prefix[x1][y2]
                        - mackerel_prefix[x2][y1]
                        + mackerel_prefix[x1][y1];

                    let sardines = sardine_prefix[x2][y2]
                        - sardine_prefix[x1][y2]
                        - sardine_prefix[x2][y1]
                        + sardine_prefix[x1][y1];

                    let score = mackerels as i32 - sardines as i32;
                    if score > best_score {
                        best_score = score;
                        best_rect = (x1, y1, w, h);
                    }
                }
            }
        }
    }

    let (x, y, w, h) = best_rect;

    let x1 = x * cell_size;
    let y1 = y * cell_size;
    let x2 = (x + w) * cell_size;
    let y2 = (y + h) * cell_size;

    let vertices = vec![
        (x1, y1),
        (x2, y1),
        (x2, y2),
        (x1, y2),
    ];

    println!("{}", vertices.len());

    for &(a, b) in &vertices {
        println!("{} {}", a, b);
    }
}
