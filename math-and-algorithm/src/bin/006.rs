// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_f
// 006 - Print 2N+3
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: usize = solve(n);
}

fn solve(n: usize) -> usize {
    println!("{}", 2 * n + 3);
    2 * n + 3
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 100;
        assert_eq!(203, solve(n));
    }

    #[test]
    fn test_2() {
        let n = 27;
        assert_eq!(57, solve(n));
    }
}
