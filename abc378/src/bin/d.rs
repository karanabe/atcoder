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
        h: usize,
        w: usize,
        k: u32,
        s: [String; h],
    }

    let grid: Vec<Vec<char>> = s.iter().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let pos_bit = |x: usize, y: usize| -> u128 {
        (x * w + y) as u128
    };

    for x in 0..h {
        for y in 0..w {
            if grid[x][y] == '.' {
                let mut visited = 0u128;
                visited |= 1 << pos_bit(x, y);
                dfs(
                    x,
                    y,
                    k,
                    visited,
                    &grid,
                    h,
                    w,
                    &dirs,
                    &mut count,
                    &pos_bit,
                );
            }
        }
    }

    println!("{}", count);
}

fn dfs(
    x: usize,
    y: usize,
    step_rem: u32,
    visited: u128,
    grid: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    dirs: &[(isize, isize); 4],
    count: &mut u64,
    pos_bit: &dyn Fn(usize, usize) -> u128,
) {
    if step_rem == 0 {
        *count += 1;
        return;
    }

    for &(dx, dy) in dirs {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < h as isize && ny >= 0 && ny < w as isize {
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] == '.' {
                let pos = pos_bit(nx, ny);
                if visited & (1 << pos) == 0 {
                    let new_visited = visited | (1 << pos);
                    dfs(
                        nx,
                        ny,
                        step_rem - 1,
                        new_visited,
                        grid,
                        h,
                        w,
                        dirs,
                        count,
                        pos_bit,
                    );
                }
            }
        }
    }
}
