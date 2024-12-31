// https://atcoder.jp/contests/adt_all_20240118_3/tasks/abc233_c
// E - Product
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [[usize]; n],
    }
    let _: String = solve(n, x, a);
}

fn solve(n: usize, x: usize, a: Vec<Vec<usize>>) -> String {
    let mut ans: usize = 0;
    dfs(0, 1, n, x, &a, &mut ans);

    println!("{ans}");
    format!("{ans}")
}

fn dfs(pos: usize, result: usize, n: usize, x: usize, a: &Vec<Vec<usize>>, ans: &mut usize) {
    if pos == n {
        if result == x {
            *ans += 1;
        }
        return;
    }
    for &arr in &a[pos] {
        if result > x / arr {
            continue;
        }
        dfs(pos+1, result * arr, n, x, a, ans);
    }
}

#[cfg(test)]
mod adt_20240118_03 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 2;
        let x: usize = 40;
        let a: Vec<Vec<usize>> = vec![vec![3, 1, 8, 4], vec![2, 10, 5]];
        assert_eq!("2", solve(n, x, a));
    }

    #[test]
    fn test_2() {
        let n: usize = 3;
        let x: usize = 200;
        let a: Vec<Vec<usize>> = vec![vec![3, 10, 10, 10], vec![3, 10, 10, 10], vec![5, 2, 2, 2, 2, 2]];
        assert_eq!("45", solve(n, x, a));
    }

    #[test]
    fn test_3() {
        let n: usize = 3;
        let x: usize = 1000000000000000000;
        let a: Vec<Vec<usize>> = vec![vec![2, 1000000000, 1000000000], vec![2, 1000000000, 1000000000], vec![2, 1000000000, 1000000000]];
        assert_eq!("0", solve(n, x, a));
    }
}
