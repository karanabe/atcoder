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

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            a: [u64; n],
            b: [u64; n],
        }

        let mut ab = a.iter().zip(b.iter()).collect::<Vec<_>>();
        ab.sort_by_key(|&(ai, _)| *ai);

        let mut heap = BinaryHeap::new();
        let mut sum_bi = 0u64;
        let mut min_value = std::u128::MAX;

        for &(ai, bi) in ab.iter() {
            heap.push(*bi);
            sum_bi += *bi;

            if heap.len() > k {
                if let Some(removed_bi) = heap.pop() {
                    sum_bi -= removed_bi;
                }
            }

            if heap.len() == k {
                let value = (*ai as u128) * (sum_bi as u128);
                if value < min_value {
                    min_value = value;
                }
            }
        }

        println!("{}", min_value);
    }
}
