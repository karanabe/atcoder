// https://atcoder.jp/contebts/abc352/tasks/abc352_d
// D - Permutation Subsequence

use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }
    let _: String = solve(n, k, p);
}

fn solve(n: usize, k:usize, p: Vec<usize>) -> String {

    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i]] = i;
    }

    let mut bt = BTreeSet::new();
    for i in 0..k {
        bt.insert(q[i]);
    }

    let mut ans = bt.iter().next_back().unwrap() - bt.iter().next().unwrap();
    for i in k..n {
        bt.remove(&q[i - k]);
        bt.insert(q[i]);
        ans = ans.min(bt.iter().next_back().unwrap() - bt.iter().next().unwrap());
    }
    println!("{}", ans);
    format!("{}", ans)
}


#[cfg(test)]
mod abc352 {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let k = 2;
        let mut p = vec![2, 3, 1, 4];
        for value in p.iter_mut() { *value -= 1 }
        assert_eq!("1", solve(n, k, p));
    }

    #[test]
    fn test_2() {
        let n = 4;
        let k = 1;
        let mut p = vec![2, 3, 1, 4];
        for value in p.iter_mut() { *value -= 1 }
        assert_eq!("0", solve(n, k, p));
    }

    #[test]
    fn test_3() {
        let n = 10;
        let k = 5;
        let mut p = vec![10, 1, 6, 8, 7, 2, 5, 9, 3, 4];
        for value in p.iter_mut() { *value -= 1 }
        assert_eq!("5", solve(n, k, p));
    }
}
