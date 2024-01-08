// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_h
// 008 - Brute Force 1
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
    }
    let _: usize = solve(n, s);
}

fn solve(n: usize, s: usize) -> usize {
    let mut result = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                result += 1;
            }
        }
    }
    println!("{result}");
    result
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let s = 4;
        assert_eq!(6, solve(n, s));
    }

    #[test]
    fn test_2() {
        let n = 869;
        let s = 120;
        assert_eq!(7140, solve(n, s));
    }
}
