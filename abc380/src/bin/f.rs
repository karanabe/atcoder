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
    BinaryHeap,
    hash_map::Entry,
};

#[allow(unused_imports)]
use rustc_hash::FxHashMap;

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
        l: usize,
        a: [u32; n],
        b: [u32; m],
        c: [u32; l],
    }

    let mut total_cards = Vec::with_capacity(n + m + l);

    total_cards.extend_from_slice(&a);
    total_cards.extend_from_slice(&b);
    total_cards.extend_from_slice(&c);

    let mut state = vec![1; n];
    state.extend(vec![2; m]);
    state.extend(vec![0; l]);

    let mut memo: HashMap<Vec<u32>, bool> = HashMap::new();
    if is_first_player_win(&total_cards, state, &mut memo) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn is_first_player_win(cards: &[u32], status: Vec<u32>, memo: &mut HashMap<Vec<u32>, bool>) -> bool {
    let entry = memo.entry(status.clone());

    if let Entry::Occupied(v) = entry {
        return *v.get();
    }

    let mut result = false;

    for i in 0..cards.len() {
        if status[i] == 1 {
            let card = cards[i];
            let mut s = status.clone();
            s[i] = 0;

            for j in 0..s.len() {
                if s[j] == 1 {
                    s[j] = 2;
                } else if s[j] == 2 {
                    s[j] = 1;
                }
            }

            result |= !is_first_player_win(cards, s.clone(), memo);

            for j in 0..status.len() {
                if s[j] == 0 && cards[j] < card {
                    let mut ss = s.clone();
                    ss[j] = 2;
                    result |= !is_first_player_win(cards, ss, memo);
                }
            }
        }
    }

    memo.insert(status, result);

    return result;
}
