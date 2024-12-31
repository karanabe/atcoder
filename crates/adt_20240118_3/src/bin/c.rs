// https://atcoder.jp/contests/adt_all_20240118_3/tasks/abc226_b
// C - Counting Arrays
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [[usize]; n],
    }
    let _: String = solve(n, a);
}

fn solve(_n: usize, a: Vec<Vec<usize>>) -> String {
    let mut result = BTreeSet::new();
    // Another answer
    // l.iter().collect::<HashSet<_>>().len()
    // l.iter().unique().count()
    // a.sort(); a.dedup();
    for arr in a {
        result.insert(arr);
    }
    println!("{}", result.len());
    format!("{}", result.len())
}

#[cfg(test)]
mod adt_20240118_03 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 4;
        let a: Vec<Vec<usize>> = vec![vec![2, 1, 2], vec![2, 1, 1], vec![2, 2, 1], vec![2, 1, 2]];
        assert_eq!("3", solve(n, a));
    }

    #[test]
    fn test_2() {
        let n: usize = 5;
        let a: Vec<Vec<usize>> = vec![vec![1, 1], vec![1, 1], vec![1, 2], vec![2, 1, 1], vec![3, 1, 1, 1]];
        assert_eq!("4", solve(n, a));
    }

    #[test]
    fn test_3() {
        let n: usize = 1;
        let a: Vec<Vec<usize>> = vec![vec![1, 1]];
        assert_eq!("1", solve(n, a));
    }
}
