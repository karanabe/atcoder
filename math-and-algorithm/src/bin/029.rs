// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ab
// 029 - Climb Stairs
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: String = solve(n);
}

fn solve(n: usize) -> String {
    let mut dp: Vec<i32> = vec![0; n+1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    println!("{}", dp[n]);
    format!("{}", dp[n])
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        assert_eq!("5", solve(n));
    }

    #[test]
    fn test_2() {
        let n = 6;
        assert_eq!("13", solve(n));
    }

    #[test]
    fn test_3() {
        let n = 45;
        assert_eq!("1836311903", solve(n));
    }
}
