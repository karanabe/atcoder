// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_b
// 002 - Sum of 3 Integers
use proconio::input;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }
    let _: usize = solve(a1, a2, a3);
}

fn solve(a1: usize, a2: usize, a3: usize) -> usize {
    println!("{}", a1 + a2 + a3);
    a1 + a2 + a3
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let a1 = 10;
        let a2 = 20;
        let a3 = 50;
        assert_eq!(80, solve(a1, a2, a3));
    }

    #[test]
    fn test_2() {
        let a1 = 1;
        let a2 = 1;
        let a3 = 1;
        assert_eq!(3, solve(a1, a2, a3));
    }

    #[test]
    fn test_3() {
        let a1 = 100;
        let a2 = 100;
        let a3 = 100;
        assert_eq!(300, solve(a1, a2, a3));
    }
}
