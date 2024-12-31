// https://atcoder.jp/contests/adt_all_20240118_3/tasks/abc240_b
// D - Count Distinct Integers
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: String = solve(n, a);
}

fn solve(_n: usize, a: Vec<usize>) -> String {
    let result = a.iter().collect::<BTreeSet<_>>().len();
    println!("{result}");
    format!("{result}")
}

#[cfg(test)]
mod adt_20240118_03 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 6;
        let a: Vec<usize> = vec![1, 4, 1, 2, 2, 1];
        assert_eq!("3", solve(n, a));
    }

    #[test]
    fn test_2() {
        let n: usize = 1;
        let a: Vec<usize> = vec![1];
        assert_eq!("1", solve(n, a));
    }

    #[test]
    fn test_3() {
        let n: usize = 11;
        let a: Vec<usize> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        assert_eq!("7", solve(n, a));
    }
}
