// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i
// 009 - Brute Force 2
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let _: String = solve(n, s, a);
}

fn solve(n: usize, s:usize, a: Vec<usize>) -> String {

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;

    // dp[][If sum is equal to j] = true
    // test case: test_1
    // i=2; j=11; a[i]=9
    // dp[][11-9] -> dp[][2] = true
    // dp[][2] -> i=0; j=2; a[0]=2 -> finally j - a[i] = 0
    // dp[0][0] = true
    // This mean a[0] + a[2] = 11; s=11
    for i in 0..n {
        for j in 0..=s {
            // break point here to look into how work this dp
            if dp[i][j] || (a[i] <= j && dp[i][j - a[i]]) {
                dp[i + 1][j] = true;
            }
        }
    }
    println!("{}", if dp[n][s] {"Yes"} else {"No"});
    format!("{}", if dp[n][s] {"Yes"} else {"No"})
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let s = 11;
        let a = vec![2, 5, 9];
        assert_eq!("Yes", solve(n, s, a));
    }

    #[test]
    fn test_2() {
        let n = 4;
        let s = 11;
        let a = vec![3, 1, 4, 5];
        assert_eq!("No", solve(n, s, a));
    }

}
