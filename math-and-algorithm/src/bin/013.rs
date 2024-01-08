// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_m
// 013 - Divisor Enumeration
use proconio::input;
// use std::collections::HashSet;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: f64,
    }
    solve(n);
}

fn solve(n: f64) {
    let mut result: BTreeSet<usize> = BTreeSet::new();
    for i in 1..=(n.sqrt() as usize) {
        if n as usize % i == 0 {
            result.insert(i);
            result.insert(n as usize / i);
        }
    }
    for j in result {
        println!("{j}");
    }
}
