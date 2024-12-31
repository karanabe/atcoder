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
        n: usize,
        m: usize,
        intervals: [(usize, usize); n],
    }

    let mut min_r = vec![m + 1; m + 2];

    let mut starts = vec![Vec::new(); m + 2];
    for (li, ri) in intervals {
        starts[li].push(ri);
    }

    for l in (1..=m).rev() {
        for &ri in &starts[l] {
            min_r[l] = min(min_r[l], ri);
        }
        min_r[l] = min(min_r[l], min_r[l + 1]);
    }

    let mut ans: i64 = 0;

    for l in 1..=m {
        let cnt_l = min_r[l].saturating_sub(l) as i64;
        ans += cnt_l;
    }

    println!("{}", ans);
}
