#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    input! {
        mut n: usize,
    }

    let mut ans = 0;

    while 10000 <= n {
        n -= 10000;
        ans += 1;
    }
    while 5000 <= n {
        n -= 5000;
        ans += 1;
    }
    while 1000 <= n {
        n -= 1000;
        ans += 1;
    }

    println!("{ans}");
}
