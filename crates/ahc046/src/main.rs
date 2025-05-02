#[allow(unused_imports)]
use proconio::{
    input,
    input_interactive,
    fastout,
    source::line::LineSource,
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
    max,
    Ordering
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

#[allow(unused_imports)]
use std::io::{self, Write, BufReader, StdinLock};

#[allow(unused_imports)]
use rand::{
    rngs::StdRng,
    Rng,
    thread_rng,
    SeedableRng,
    seq::SliceRandom,
    prelude::*,
};
#[allow(unused_imports)]
use rand_distr::{Normal, Distribution};
#[allow(unused_imports)]
use std::time::{Instant, Duration};


const N: usize = 20;
const DIR: [(isize, isize, char); 4] = [(-1, 0, 'U'), (1, 0, 'D'), (0, -1, 'L'), (0, 1, 'R')];

#[derive(Clone, Copy)]
struct Step {
    y: usize,
    x: usize,
    // (py, px, action, dir)
    from: Option<(usize, usize, char, char)>,
}

fn bfs(start: (usize, usize), goal: (usize, usize)) -> Vec<(char, char)> {
    let mut q = VecDeque::new();
    let mut vis = [[false; N]; N];
    let mut prev: Vec<Vec<Option<Step>>> = vec![vec![None; N]; N];

    q.push_back((start.0, start.1));
    vis[start.0][start.1] = true;
    prev[start.0][start.1] = Some(Step { y: start.0, x: start.1, from: None });

    while let Some((y, x)) = q.pop_front() {
        if (y, x) == goal {
            break;
        }
        // move 1 step
        for (dy, dx, dch) in DIR {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0 && ny < N as isize && nx >= 0 && nx < N as isize {
                let (nyu, nxu) = (ny as usize, nx as usize);
                if !vis[nyu][nxu] {
                    vis[nyu][nxu] = true;
                    prev[nyu][nxu] = Some(Step { y: nyu, x: nxu, from: Some((y, x, 'M', dch)) });
                    q.push_back((nyu, nxu));
                }
            }
        }
        // slide
        for (dy, dx, dch) in DIR {
            let mut ny = y as isize;
            let mut nx = x as isize;
            loop {
                let ty = ny + dy;
                let tx = nx + dx;
                if ty < 0 || ty >= N as isize || tx < 0 || tx >= N as isize {
                    break;
                }
                ny = ty;
                nx = tx;
            }
            let (nyu, nxu) = (ny as usize, nx as usize);
            if !vis[nyu][nxu] {
                vis[nyu][nxu] = true;
                prev[nyu][nxu] = Some(Step { y: nyu, x: nxu, from: Some((y, x, 'S', dch)) });
                q.push_back((nyu, nxu));
            }
        }
    }

    let mut seq = Vec::<(char, char)>::new();
    let mut cur = goal;
    while cur != start {
        if let Some(st) = &prev[cur.0][cur.1] {
            let (py, px, act, dir) = st.from.unwrap();
            seq.push((act, dir));
            cur = (py, px);
        }
    }
    seq.reverse();
    seq
}

fn main() {
    input! {
        n: usize, m: usize,
        points: [(usize, usize); m],
    }
    let mut cur = points[0];
    let mut out: Vec<(char, char)> = Vec::new();
    for &next in &points[1..] {
        out.extend(bfs(cur, next));
        cur = next;
    }

    for (a, d) in out {
        println!("{} {}", a, d);
    }
}
