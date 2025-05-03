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
        n: usize,
        k_list: [i64; n],
    }

    let ttl_sum: i64 = k_list.iter().sum();
    let mut min_max_sum = ttl_sum;

    for bit in 0..(1 << n) {
        let mut a_sum = 0i64;
        let mut b_sum = 0i64;

        for i in 0..n {
            if (bit & (1 << i)) != 0 {
                a_sum += k_list[i];
            } else {
                b_sum += k_list[i];
            }
        }

        let current_max = a_sum.max(b_sum);
        if current_max < min_max_sum {
            min_max_sum = current_max;
        }
    }

    println!("{}", min_max_sum);
}
