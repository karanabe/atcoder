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
