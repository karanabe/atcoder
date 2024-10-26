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
        x: i64,
        mach: [(i64, i64, i64, i64); n],
    }

    let mut left = 0i64;
    let mut right = 1_000_000_000i64;

    while left < right {
        let mid = (left + right + 1) / 2;

        let mut total_cost = 0i64;
        let mut possible = true;

        for &(ai, pi, bi, qi) in &mach {
            let cost = min_cost(ai, pi, bi, qi, mid);
            if cost.is_none() {
                possible = false;
                break;
            } else {
                total_cost += cost.unwrap();
            }

            if total_cost > x {
                possible = false;
                break;
            }
        }

        if possible && total_cost <= x {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", left);
}


fn min_cost(ai: i64, pi: i64, bi: i64, qi: i64, w: i64) -> Option<i64> {
    let mut m_cost = None;

    let delta = 100;

    let ni_t_base = w / bi;
    let ni_t_start = if ni_t_base >= delta { ni_t_base - delta } else { 0 };
    let ni_t_end = ni_t_base + delta;

    for ni_t in ni_t_start..=ni_t_end {
        let rem_c = w - bi * ni_t;
        let ni_s = if rem_c > 0 {
            (rem_c + ai - 1) / ai
        } else {
            0
        };

        let cost = pi * ni_s + qi * ni_t;
        if m_cost.is_none() || cost < m_cost.unwrap() {
            m_cost = Some(cost);
        }
    }

    let ni_s_base = w / ai;
    let ni_s_start = if ni_s_base >= delta { ni_s_base - delta } else { 0 };
    let ni_s_end = ni_s_base + delta;

    for ni_s in ni_s_start..=ni_s_end {
        let rem_c = w - ai * ni_s;
        let ni_t = if rem_c > 0 {
            (rem_c + bi - 1) / bi
        } else {
            0
        };

        let cost = pi * ni_s + qi * ni_t;
        if m_cost.is_none() || cost < m_cost.unwrap() {
            m_cost = Some(cost);
        }
    }

    m_cost
}
