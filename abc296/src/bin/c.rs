// https://atcoder.jp/contests/abc296/tasks/abc296_c
// C - Gap Existence
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }
    let _: String = solve(n, x, &a);
}

fn solve(_n: usize, x: i64, a: &[i64]) -> String {
    let hs = a.iter().map(|j| j + x).collect::<HashSet<i64>>();
    let result = if a.iter().any(|i| hs.contains(i)) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", result);
    result.to_string()
}

#[cfg(test)]
mod abc296 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 6;
        let x: i64 = 5;
        let a: [i64; 6] = [3, 1, 4, 1, 5, 9];
        assert_eq!("Yes", solve(n, x, &a));
    }

    #[test]
    fn test_2() {
        let n: usize = 6;
        let x: i64 = -4;
        let a: [i64; 6] = [-2, -7, -1, -8, -2, -8];
        assert_eq!("No", solve(n, x, &a));
    }

    #[test]
    fn test_3() {
        let n: usize = 2;
        let x: i64 = 0;
        let a: [i64; 2] = [141421356, 17320508];
        assert_eq!("Yes", solve(n, x, &a));
    }
}
