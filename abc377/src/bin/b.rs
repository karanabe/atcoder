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
        grid: [String; 8],
    }

    let mut rows = [false; 8];
    let mut cols = [false; 8];

    let grid_chars: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();

    for i in 0..8 {
        for j in 0..8 {
            if grid_chars[i][j] == '#' {
                rows[i] = true;
                cols[j] = true;
            }
        }
    }

    let mut ans = 0;

    for i in 0..8 {
        for j in 0..8 {
            if grid_chars[i][j] == '.' && !rows[i] && !cols[j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
