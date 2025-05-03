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
        s: String,
        t: String,
    }

    let min_len = s.len().min(t.len());

    for (i, (sc, tc)) in s.chars().zip(t.chars()).enumerate() {
        if sc != tc {
            println!("{}", i + 1);
            return;
        }
    }

    if s.len() != t.len() {
        println!("{}", min_len + 1);
    } else {
        println!("0");
    }
}
