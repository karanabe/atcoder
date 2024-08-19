// https://atcoder.jp/contests/abc355/tasks/abc355_d
// D - Intersecting Intervals
use proconio::input;
use std::cmp;

struct FenwickTree {
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree {
            data: vec![0; size + 1],
        }
    }

    fn add(&mut self, mut idx: usize, value: i64) {
        while idx < self.data.len() {
            self.data[idx] += value;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn sum(&self, mut idx: usize) -> i64 {
        let mut result = 0;
        while idx > 0 {
            result += self.data[idx];
            idx &= idx - 1;
        }
        result
    }
}

fn main() {
    input! {
        n: usize,
        intervals: [(usize, usize); n],
    }

    let mut events = Vec::new();

    for &(l, r) in &intervals {
        events.push((l, 1)); // 開始点
        events.push((r, -1)); // 終点
    }

    events.sort();

    let mut ft = FenwickTree::new(1_000_000_010);
    let mut result = 0;

    for &(x, type_) in &events {
        if type_ == 1 {
            result += ft.sum(x);
        }
        ft.add(x, type_ as i64);
    }

    println!("{}", result);
}
