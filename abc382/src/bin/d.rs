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


fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = Vec::new();
    let max_xx = m.saturating_sub(10 * (n - 1));

    for xx in 1..=max_xx {
        let mut seq = vec![xx];
        dfs(n, m, 2, &mut seq, &mut ans);
    }

    println!("{}", ans.len());
    for seq in ans {
        println!("{}", seq.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}

fn dfs(
    n: usize,
    m: usize,
    pos: usize,
    seq: &mut Vec<usize>,
    ans: &mut Vec<Vec<usize>>,
) {
    if pos > n {
        if *seq.last().unwrap() <= m {
            ans.push(seq.clone());
        }
        return;
    }

    let prev = *seq.last().unwrap();
    let min_ai = prev + 10;
    let max_ai = m.saturating_sub(10 * (n - pos));

    if min_ai > max_ai {
        return;
    }

    for ai in min_ai..=max_ai {
        seq.push(ai);
        dfs(n, m, pos + 1, seq, ans);
        seq.pop();
    }
}

