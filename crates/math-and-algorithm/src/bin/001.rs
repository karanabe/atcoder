// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_a
// 001 - Print 5+N
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: usize = solve(n);
}

fn solve(n: usize) -> usize {
    println!("{}", n+5);
    n + 5
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        assert_eq!(7, solve(n));
    }

    #[test]
    fn test_2() {
        let n = 4;
        assert_eq!(9, solve(n));
    }

    #[test]
    fn test_3() {
        let n = 8;
        assert_eq!(13, solve(n));
    }
}
