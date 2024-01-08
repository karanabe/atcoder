// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o
// 015 - Calculate GCD
use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: usize,
        b: usize
    }
    let _: usize = solve(a, b);
}

fn solve(a: usize, b: usize) -> usize {
    let mut result = 1;
    for i in 2..=min(a, b) {
        if a % i == 0 && b % i ==0 {
            result = i;
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
        assert_eq!(11, solve(33, 88));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, solve(123, 777));
    }
}
