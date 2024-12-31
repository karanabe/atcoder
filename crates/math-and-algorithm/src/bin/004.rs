// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_d
// 004 - Product of 3 Integers
use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    }
    let _: usize = solve(a);
}

fn solve(a: Vec<usize>) -> usize {
    println!("{}", a.iter().product::<usize>());
    a.iter().product::<usize>()
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let a = vec![2, 8, 8];
        assert_eq!(128, solve(a));
    }

    #[test]
    fn test_2() {
        let a = vec![7, 7, 25];
        assert_eq!(1225, solve(a));
    }

    #[test]
    fn test_3() {
        let a = vec![100, 100, 100];
        assert_eq!(1000000, solve(a));
    }
}
