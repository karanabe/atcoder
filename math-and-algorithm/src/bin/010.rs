// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j
// 010 - Factorial
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: usize = solve(n,);
}

fn solve(n: usize) -> usize {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    println!("{result}");
    result
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        assert_eq!(120, solve(n));
    }
}
