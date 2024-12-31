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
        s: f64,
        t: f64,
        lines: [(f64, f64, f64, f64); n],
    }

    let mut segments = Vec::new();
    for &(a, b, c, d) in &lines {
        segments.push(((a, b), (c, d)));
    }

    let mut min_time = std::f64::MAX;

    let permutations = permute((0..n).collect());

    for perm in permutations {
        for bits in 0..(1 << n) {
            let mut time = 0.0;
            let mut curr_pos = (0.0, 0.0);

            for (idx, &line_idx) in perm.iter().enumerate() {
                let segment = &segments[line_idx];
                let start_pos;
                let end_pos;

                if (bits & (1 << idx)) == 0 {
                    start_pos = segment.0;
                    end_pos = segment.1;
                } else {
                    start_pos = segment.1;
                    end_pos = segment.0;
                }

                let distance_to_start = distance(curr_pos, start_pos);
                time += distance_to_start / s;

                let segment_length = distance(start_pos, end_pos);
                time += segment_length / t;

                curr_pos = end_pos;
            }

            if time < min_time {
                min_time = time;
            }
        }
    }

    println!("{:.15}", min_time);
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

fn permute(nums: Vec<usize>) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    let mut nums = nums;
    heap_permutation(nums.len(), &mut nums, &mut results);
    results
}

fn heap_permutation(k: usize, nums: &mut [usize], results: &mut Vec<Vec<usize>>) {
    if k == 1 {
        results.push(nums.to_vec());
    } else {
        for i in 0..k {
            heap_permutation(k - 1, nums, results);
            if k % 2 == 0 {
                nums.swap(i, k - 1);
            } else {
                nums.swap(0, k - 1);
            }
        }
    }
}
