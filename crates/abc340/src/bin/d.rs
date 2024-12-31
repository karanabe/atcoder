// https://atcoder.jp/contests/abc340/tasks/abc340_d
// D - Super Takahashi Bros.
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        p: [(i64, i64, Usize1); n - 1],
    }
    let _: String = solve(n, p);
}

fn solve(n: usize, p: Vec<(i64, i64, usize)>) -> String {

    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    let mut h = BinaryHeap::new();
    h.push((0, 0));
    while let Some((d, v)) = h.pop() {
        let d = -d;
        if d > dp[v] || v == n - 1 {
            continue;
        }
        let (a, b, x) = p[v];
        if d + a < dp[v + 1] {
            dp[v + 1] = d + a;
            h.push((-dp[v + 1], v + 1));
        }
        if d + b < dp[x] {
            dp[x] = d + b;
            h.push((-dp[x], x));
        }
    }
    println!("{}", dp[n - 1]);

    format!("{}", dp[n - 1])
}
