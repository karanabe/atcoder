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
        n: usize,
        s: Chars,
    }

    let mut ans = 0;

    if n < 3 {
        println!("0");
        return
    }

    for i in 0..(n - 2) {
        if s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#' {
            ans += 1;
        }
    }

    println!("{ans}");
}
