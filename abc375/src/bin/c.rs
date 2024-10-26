#[allow(unused_imports)]
use proconio::{
    input,
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

fn move_position(x: usize, y: usize, cnt: usize, n: usize) -> (usize, usize) {
    match cnt % 4 {
        0 => (x, y),
        1 => (n - y - 1, x),
        2 => (n - x - 1, n - y - 1),
        _ => (y, n - x - 1),
    }
}

fn min4(a: usize, b: usize, c: usize, d: usize) -> usize {
    min(min(a, b), min(c, d))
}

fn main() {
    input! {
        n: usize,
        a: [String; n],
    }

    let a: Vec<Vec<char>> = a.iter().map(|s| s.chars().collect()).collect();
    let mut ans = vec![vec!['.'; n]; n];

    for i in 0..n {
        for j in 0..n {
            let cnt = min4(i + 1, j + 1, n - i, n - j);
            let (x, y) = move_position(i, j, cnt, n);
            ans[i][j] = a[x][y];
        }
    }

    for row in ans {
        println!("{}", row.into_iter().collect::<String>());
    }
}
