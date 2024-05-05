// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ac
// 031 - Taro's Vacation
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: String = solve(n, a);
}

fn solve(n: usize, a: Vec<usize>) -> String {
    let mut dp: Vec<usize> = vec![0; n];

    dp[0] = a[0];
    dp[1] = a[1];

    for i in 2..n {
        if dp[i - 1] < a[i] + dp[i - 2] {
            dp[i] = a[i] + dp[i - 2]
        } else {
            dp[i] = dp[i - 1]
        }
    }

    println!("{}", dp[n - 1]);
    format!("{}", dp[n - 1])
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        let a = vec![2, 5, 3, 3, 1];
        assert_eq!("8", solve(n, a));
    }
}
