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
        q: usize,
        ht: [(char, usize); q],
    }

    let mut ans = 0;
    let mut l = 1;
    let mut r = 2;
    let mut current_hand_pos;
    let mut obstacle_pos;
    let mut target_pos;

    for (h, t) in ht {
        if h == 'R' {
            current_hand_pos = r;
            obstacle_pos = l;
            target_pos = t;
            r = t;
        } else {
            current_hand_pos = l;
            obstacle_pos = r;
            target_pos = t;
            l = t;
        }
        match distance_in_ring_buffer_with_obstacle(n,current_hand_pos, target_pos, obstacle_pos) {
            Some(value) => ans += value,
            None => {},
        }
    }

    println!("{ans}");
}

fn distance_in_ring_buffer_with_obstacle(
    buffer_len: usize,
    current_pos: usize,
    target_pos: usize,
    obstacle_pos: usize
) -> Option<usize> {

    let left_distance = (current_pos + buffer_len - target_pos) % buffer_len;
    let right_distance = (target_pos + buffer_len - current_pos) % buffer_len;

    let left_blocked = (current_pos + buffer_len - obstacle_pos) % buffer_len <= left_distance;
    let right_blocked = (obstacle_pos + buffer_len - current_pos) % buffer_len <= right_distance;

    let left_result = if left_blocked { None } else { Some(left_distance) };
    let right_result = if right_blocked { None } else { Some(right_distance) };

    match (left_result, right_result) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}
