// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_c
// 003 - Sum of N Integers
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(n, &a);
}

fn solve(_n: usize, a: &[usize]) -> usize {
    println!("{}", a.iter().sum::<usize>());
    a.iter().sum::<usize>()
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        let a = [3, 1, 4, 1, 5];
        assert_eq!(14, solve(n, &a));
    }

    #[test]
    fn test_2() {
        let n = 3;
        let a = [10, 20, 50];
        assert_eq!(80, solve(n, &a));
    }

    #[test]
    fn test_3() {
        let n = 10;
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(55, solve(n, &a));
    }
}
