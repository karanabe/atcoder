// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_a
// 028 - Frog 1
use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let _: String = solve(n, a);
}

fn solve(n: usize, a: Vec<i32>) -> String {
    let mut dp: Vec<i32> = vec![0; n];
    dp[0] = 0;
    dp[1] = (a[1] - a[0]).abs();
    for i in 2..n {
        let v1 = dp[i - 1] + (a[i - 1] - a[i]).abs();
        let v2 = dp[i - 2] + (a[i - 2] - a[i]).abs();
        dp[i] = min(v1, v2);
    }
    println!("{}", dp[n - 1]);
    format!("{}", dp[n - 1])
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let a = vec![10, 30, 40, 20];
        assert_eq!("30", solve(n, a));
    }

    #[test]
    fn test_2() {
        let n = 2;
        let a = vec![10, 10];
        assert_eq!("0", solve(n, a));
    }
}
