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

const N: usize = 20;
const MAX_DIST: i32 = (N as i32 - 1) * 2;
const CANDIDATES: usize = 200;

#[derive(Clone, Copy)]
struct BoxInfo {
    w: i32,
    durability: i32,
}

// Add damage when move 1 cell
fn apply_damage(stack: &mut [BoxInfo]) -> bool {
    let mut above = 0;
    for b in stack.iter_mut().rev() {
        b.durability -= above;
        if b.durability <= 0 {
            return false;
        }
        above += b.w;
    }
    true
}

// Check if it can be loaded
fn is_loaded(stack: &[BoxInfo], w_new: i32, dist: i32) -> bool {
    let mut above = w_new;
    for b in stack.iter().rev() {
        if b.durability <= above * dist {
            return false;
        }
        above += b.w;
    }
    true
}

// Start point(0,0) -> target -> start point
// return turns
// If record_ops=true push to ops 'U','D','L','R','1'
fn do_trip(
    target: (usize, usize),
    done: &mut [[bool; N]; N],
    w: &Vec<Vec<i32>>,
    d: &Vec<Vec<i32>>,
    record_ops: bool,
    ops: &mut Vec<char>,
) -> i32 {
    let (tx, ty) = target;
    let mut moves = 0;
    let mut x = 0i32;
    let mut y = 0i32;

    for _ in 0..ty {
        moves += 1;
        if record_ops {
            ops.push('R');
        }
        y += 1;
    }
    for _ in 0..tx {
        moves += 1;
        if record_ops {
            ops.push('D');
        }
        x += 1;
    }

    // Load target
    moves += 1;
    if record_ops {
        ops.push('1');
    }
    let mut stack = vec![BoxInfo {
        w: w[tx][ty],
        durability: d[tx][ty],
    }];
    done[tx][ty] = true;

    while x > 0 {
        moves += 1;
        if record_ops {
            ops.push('U');
        }
        x -= 1;

        // Broken
        if !apply_damage(&mut stack) {
            return 1_000_000_000;
        }

        if !done[x as usize][ty] {
            let w_new = w[x as usize][ty];
            let d_new = d[x as usize][ty];
            let dist = x + y;
            if is_loaded(&stack, w_new, dist) {
                moves += 1;
                if record_ops {
                    ops.push('1');
                }
                stack.push(BoxInfo {
                    w: w_new,
                    durability: d_new,
                });
                done[x as usize][ty] = true;
            }
        }
    }

    if !done[0][ty] {
        let w_new = w[0][ty];
        let d_new = d[0][ty];
        let dist = y;
        if is_loaded(&stack, w_new, dist) {
            moves += 1;
            if record_ops {
                ops.push('1');
            }
            stack.push(BoxInfo {
                w: w_new,
                durability: d_new,
            });
            done[0][ty] = true;
        }
    }

    while y > 0 {
        moves += 1;
        if record_ops {
            ops.push('L');
        }
        y -= 1;
        if !apply_damage(&mut stack) {
            return 1_000_000_000;
        }

        if y == 0 {
            break;
        }
        if !done[0][y as usize] {
            let w_new = w[0][y as usize];
            let d_new = d[0][y as usize];
            let dist = y;
            if is_loaded(&stack, w_new, dist) {
                moves += 1;
                if record_ops {
                    ops.push('1');
                }
                stack.push(BoxInfo {
                    w: w_new,
                    durability: d_new,
                });
                done[0][y as usize] = true;
            }
        }
    }
    moves
}

fn do_trip_rev(
    target: (usize, usize),
    done: &mut [[bool; N]; N],
    w: &Vec<Vec<i32>>,
    d: &Vec<Vec<i32>>,
    record_ops: bool,
    ops: &mut Vec<char>,
) -> i32 {
    let (tx, ty) = target;
    let mut moves = 0;
    let mut x = 0i32;
    let mut y = 0i32;

    for _ in 0..ty {
        moves += 1;
        if record_ops {
            ops.push('R');
        }
        y += 1;
    }
    for _ in 0..tx {
        moves += 1;
        if record_ops {
            ops.push('D');
        }
        x += 1;
    }

    // Load target
    moves += 1;
    if record_ops {
        ops.push('1');
    }
    let mut stack = vec![BoxInfo {
        w: w[tx][ty],
        durability: d[tx][ty],
    }];
    done[tx][ty] = true;

    while y > 0 {
        moves += 1;
        if record_ops {
            ops.push('L');
        }
        y -= 1;
        if !apply_damage(&mut stack) {
            return 1_000_000_000;
        }

        if y == 0 {
            break;
        }
        if !done[0][y as usize] {
            let w_new = w[0][y as usize];
            let d_new = d[0][y as usize];
            let dist = y;
            if is_loaded(&stack, w_new, dist) {
                moves += 1;
                if record_ops {
                    ops.push('1');
                }
                stack.push(BoxInfo {
                    w: w_new,
                    durability: d_new,
                });
                done[0][y as usize] = true;
            }
        }
    }

    if !done[0][ty] {
        let w_new = w[0][ty];
        let d_new = d[0][ty];
        let dist = y;
        if is_loaded(&stack, w_new, dist) {
            moves += 1;
            if record_ops {
                ops.push('1');
            }
            stack.push(BoxInfo {
                w: w_new,
                durability: d_new,
            });
            done[0][ty] = true;
        }
    }

    while x > 0 {
        moves += 1;
        if record_ops {
            ops.push('U');
        }
        x -= 1;

        // Broken
        if !apply_damage(&mut stack) {
            return 1_000_000_000;
        }

        if !done[x as usize][ty] {
            let w_new = w[x as usize][ty];
            let d_new = d[x as usize][ty];
            let dist = x + y;
            if is_loaded(&stack, w_new, dist) {
                moves += 1;
                if record_ops {
                    ops.push('1');
                }
                stack.push(BoxInfo {
                    w: w_new,
                    durability: d_new,
                });
                done[x as usize][ty] = true;
            }
        }
    }

    moves
}

fn simulate_total(done_init: &[[bool; N]; N], w: &Vec<Vec<i32>>, d: &Vec<Vec<i32>>) -> i32 {
    let mut done = *done_init;
    let mut total_moves = 0;

    loop {
        let mut target = None;
        'outer: for dist in (1..=MAX_DIST).rev() {
            for i in 0..=dist as usize {
                let j = dist as usize - i;
                if i >= N || j >= N {
                    continue;
                }
                if !done[i][j] {
                    target = Some((i, j));
                    break 'outer;
                }
            }
        }
        let Some(t) = target else { break };
        total_moves += do_trip(t, &mut done, w, d, false, &mut Vec::new());
        if total_moves >= 1_000_000_000 {
            break;
        }
    }
    total_moves
}

// Tries at most CANDIDATES from furthest away and returns best target
fn choose_target(done: &[[bool; N]; N], w: &Vec<Vec<i32>>, d: &Vec<Vec<i32>>) -> (usize, usize) {
    let mut cand: Vec<(usize, usize)> = Vec::with_capacity(CANDIDATES);

    'dist: for dist in (1..=MAX_DIST).rev() {
        for i in 0..=dist as usize {
            let j = dist as usize - i;
            if i >= N || j >= N {
                continue;
            }
            if !done[i][j] {
                cand.push((i, j));
                if cand.len() == CANDIDATES {
                    break 'dist;
                }
            }
        }
    }

    let mut best_t = cand[0];
    let mut best_score = i32::MAX;

    for &t in &cand {
        let mut done_tmp = *done;
        let mut dummy_ops = Vec::new();
        let moves_first = do_trip(t, &mut done_tmp, w, d, false, &mut dummy_ops);
        if moves_first >= 1_000_000_000 {
            continue;
        }
        let rest = simulate_total(&done_tmp, w, d);
        let score = moves_first + rest;
        if score < best_score {
            best_score = score;
            best_t = t;
        }
    }
    best_t
}

fn main() {
    input! {
        _n: usize,
        weight_list: [[i32; N]; N],
        durability_list: [[i32; N]; N],
    }

    let mut done = [[false; N]; N];
    done[0][0] = true;

    let mut ops: Vec<char> = Vec::with_capacity(16_000);

    while done.iter().any(|row| row.iter().any(|&b| !b)) {
        let target = choose_target(&done, &weight_list, &durability_list);
        do_trip(
            target,
            &mut done,
            &weight_list,
            &durability_list,
            true,
            &mut ops,
        );
    }

    for c in ops {
        println!("{}", c);
    }
}
