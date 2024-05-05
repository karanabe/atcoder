// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_d
// 030 - Knapsack 1
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let _: String = solve(n, w, wv);
}

fn solve(_n: usize, w:usize, wv: Vec<(usize, usize)>) -> String {
    // dp[i] i is weight, dp[i]=value
    let mut dp: Vec<usize> = vec![0; w+1];

    // calc from i=8
    //  i | 3, 30 | 4, 50 | 5, 60
    //  8 |  30   |  80   |  90
    //  7 |  30   |  80   |  80
    //  6 |  30   |  50   |  60
    //  5 |  30   |  50   |  60
    //  4 |  30   |  50   |  50
    //  3 |  30   |  30   |  30
    //  2 |   0   |   0   |   0
    //  1 |   0   |   0   |   0
    //  0 |   0   |   0   |   0
    for (wi, v) in wv {
        for i in (wi..w+1).rev() {
            dp[i] = max(dp[i - wi] + v, dp[i]);
        }
    }
    println!("{}", dp[w]);
    format!("{}", dp[w])
}
