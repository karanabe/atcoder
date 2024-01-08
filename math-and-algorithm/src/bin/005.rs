// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_e
// 005 - Modulo 100
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(a);
}

fn solve(a: Vec<usize>) -> usize {
    println!("{}", a.iter().sum::<usize>() % 100);
    a.iter().sum::<usize>() % 100
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let a = vec![30, 50, 70];
        assert_eq!(50, solve(a));
    }

    #[test]
    fn test_2() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(55, solve(a));
    }

    #[test]
    fn test_3() {
        let a = vec![60, 60, 60, 60, 60];
        assert_eq!(0, solve(a));
    }
}
