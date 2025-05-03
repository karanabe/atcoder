#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

const DIJ: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)];

fn main() {
    input! {
        r: usize,
        c: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        a: [Chars; r],
    }
    let mut todo: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    todo.push_back((0, s));
    let mut seen = vec![vec![false; c]; r];

    while let Some((d, pos)) = todo.pop_front() {
        if seen[pos.0][pos.1] {
            continue;
        }
        if pos == g {
            println!("{}", d);
            break;
        }
        seen[pos.0][pos.1] = true;
        for &dij in &DIJ {
            let next = (pos.0.wrapping_add(dij.0), pos.1.wrapping_add(dij.1));
            if next.0 < r && next.1 < c && a[next.0][next.1] == '.' {
                todo.push_back((d + 1, next))
            }
        }
    }
}
