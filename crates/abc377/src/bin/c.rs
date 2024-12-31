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

#[fastout]
fn main() {
    input! {
        n: i64,
        m: usize,
        ab: [(i64, i64); m],
    }

    let placed_pos: HashSet<(i64, i64)> = ab.into_iter().collect();

    let moves = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];

    let mut get_pos: HashSet<(i64, i64)> = HashSet::new();

    for &(i, j) in &placed_pos {
        for &(di, dj) in &moves {
            let i_new = i + di;
            let j_new = j + dj;
            if i_new >= 1 && i_new <= n && j_new >= 1 && j_new <= n {
                let pos = (i_new, j_new);
                if !placed_pos.contains(&pos) {
                    get_pos.insert(pos);
                }
            }
        }
    }

    let total_cells = n * n;
    let knight_attack = m as i64 + get_pos.len() as i64;
    let ans = total_cells - knight_attack;

    println!("{}", ans);
}
