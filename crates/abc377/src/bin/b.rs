#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{max, min};

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
