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

#[allow(unused_variables)]
#[allow(dead_code)]
struct Interval {
    start: usize,
    end: usize,
    color: usize,
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut cnt = vec![0; n + 2];
    let mut intervals = BTreeMap::new();

    for i in 1..=n {
        intervals.insert(i, Interval { start: i, end: i, color: i });
        cnt[i] = 1;
    }

    let mut results = Vec::new();

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                x: usize,
                c: usize,
            }

            let mut it = intervals.range(..=x);
            let (&start, interval) = it.next_back().unwrap();
            if x >= start && x <= interval.end {
                let mut l = start;
                let mut r = interval.end;
                let c0 = interval.color;

                cnt[c0] -= r - l + 1;
                cnt[c] += r - l + 1;

                intervals.insert(l, Interval { start: l, end: r, color: c });

                if let Some((&prev_start, prev_interval)) = intervals.range(..l).next_back() {
                    if prev_interval.end == l - 1 && prev_interval.color == c {
                        let new_start = prev_start;
                        let new_end = r;
                        intervals.remove(&prev_start);
                        intervals.remove(&l);
                        intervals.insert(new_start, Interval { start: new_start, end: new_end, color: c });
                        l = new_start;
                        r = new_end;
                    }
                }

                if let Some(next_interval) = intervals.get(&(r + 1)) {
                    if next_interval.color == c {

                        let new_start = l;
                        let new_end = next_interval.end;
                        intervals.remove(&l);
                        intervals.remove(&(r + 1));
                        intervals.insert(new_start, Interval { start: new_start, end: new_end, color: c });
                    }
                }
            }
        } else if query == 2 {
            input! {
                c: usize,
            }
            results.push(cnt[c]);
        }
    }

    for result in results {
        println!("{}", result);
    }
}
