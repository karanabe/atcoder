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
    }

    let chars: Vec<char> = s.chars().collect();
    let _n = chars.len();

    let mut pos: HashMap<char, Vec<usize>> = HashMap::new();

    for (idx, &c) in chars.iter().enumerate() {
        pos.entry(c).or_default().push(idx);
    }

    let mut ans: u64 = 0;

    for (_c, pos_list) in pos.iter() {
        let mut sum_pos: u64 = 0;
        let mut total_c: u64 = 0;

        for (k, &pos_k) in pos_list.iter().enumerate() {
            let k = k as u64;
            let pos_k = pos_k as u64;

            if k > 0 {
                total_c += pos_k * k - sum_pos - k;
            }

            sum_pos += pos_k;
        }

        ans += total_c;
    }

    println!("{}", ans);
}
