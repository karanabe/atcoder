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
    math,
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
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
    Max,
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&mut self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree,
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

#[allow(unused_imports)]
use std::io::{self, BufReader, StdinLock, Write};

#[allow(unused_imports)]
use rand::{prelude::*, rngs::StdRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};
#[allow(unused_imports)]
use rand_distr::{Distribution, Normal};
#[allow(unused_imports)]
use std::time::{Duration, Instant};

// Direction and move command
const DIRS: [(i32, i32, char); 4] = [(-1, 0, 'U'), (1, 0, 'D'), (0, -1, 'L'), (0, 1, 'R')];

fn main() {
    input! {
        n: usize,
        m: usize,
        (si, sj): (usize, usize),
        goals: [(usize, usize); m-1],
    }

    // Block coord
    let mut blocks = vec![vec![false; n]; n];

    let mut current = (si, sj);

    // Store the all actions
    let mut all_actions: Vec<(char, char)> = Vec::new();

    for &goal in &goals {
        if let Some(path) = bfs_with_sliding(n, &blocks, current, goal) {
            all_actions.extend(path.iter());
            current = goal;

            // Block placed
            let nx = goal.0 as i32 + DIRS[0].0;
            let ny = goal.1 as i32 + DIRS[0].1;
            if in_bounds(nx, ny, n)
                && in_bounds2(nx, ny, n)
                && !goals.contains(&(nx as usize, ny as usize))
            {
                blocks[nx as usize][ny as usize] = !blocks[nx as usize][ny as usize];
                all_actions.push(('A', DIRS[0].2));
            }
        } else {
            break;
        }
    }

    for &(act, dirc) in &all_actions {
        println!("{} {}", act, dirc);
    }
}

fn bfs_with_sliding(
    n: usize,
    blocks: &[Vec<bool>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(char, char)>> {
    let mut dist = vec![vec![None; n]; n];
    let mut que = VecDeque::new();

    // Store the previous coord. (x, y)
    let mut prev = vec![vec![None; n]; n];

    // Store the previous action. (M, U)
    let mut prev_act = vec![vec![None; n]; n];

    dist[start.0][start.1] = Some(0);
    que.push_back(start);

    while let Some((x, y)) = que.pop_front() {
        let d = dist[x][y].unwrap();
        if (x, y) == goal {
            break;
        }
        for &(dx, dy, dir_char) in &DIRS {
            // M Move
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if in_bounds(nx, ny, n) {
                let (ux, uy) = (nx as usize, ny as usize);

                // Check the cell is not block and if never reached
                if !blocks[ux][uy] && dist[ux][uy].is_none() {
                    dist[ux][uy] = Some(d + 1);
                    prev[ux][uy] = Some((x, y));
                    prev_act[ux][uy] = Some(('M', dir_char));
                    que.push_back((ux, uy));
                }
            }

            // S slide
            let mut sx = x as i32;
            let mut sy = y as i32;

            // Slide to bounds or blocks
            loop {
                let tx = sx + dx;
                let ty = sy + dy;
                if !in_bounds(tx, ty, n) {
                    break;
                }
                let (u_tx, u_ty) = (tx as usize, ty as usize);
                if blocks[u_tx][u_ty] {
                    break;
                }
                sx = tx;
                sy = ty;
            }

            if sx != x as i32 || sy != y as i32 {
                let (ux, uy) = (sx as usize, sy as usize);

                // If never reached
                if dist[ux][uy].is_none() {
                    dist[ux][uy] = Some(d + 1);
                    prev[ux][uy] = Some((x, y));
                    prev_act[ux][uy] = Some(('S', dir_char));
                    que.push_back((ux, uy));
                }
            }
        }
    }

    // If not reach to goal, return None
    dist[goal.0][goal.1]?;

    // restore the route
    let mut path = Vec::new();
    let mut cur = goal;
    while cur != start {
        let (pi, pj) = prev[cur.0][cur.1].unwrap();
        let (act, dirc) = prev_act[cur.0][cur.1].unwrap();
        path.push((act, dirc));
        cur = (pi, pj);
    }
    path.reverse();
    Some(path)
}

#[inline]
fn in_bounds(x: i32, y: i32, n: usize) -> bool {
    x >= 0 && x < n as i32 && y >= 0 && y < n as i32
}

#[inline]
fn in_bounds2(x: i32, y: i32, n: usize) -> bool {
    x >= 3 && x < n as i32 - 3 && y >= 3 && y < n as i32 - 3
}
