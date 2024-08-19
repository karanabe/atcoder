// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ad
// 032 - Binary Search
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let _: String = solve(n, x, a);
}

fn solve(n: usize, x: usize, mut a: Vec<usize>) -> String {
    a.sort();

    let mut ans = "No";

    let mut left = 1;
    let mut right = n-1;

    while left <= right {
        let mid = (left + right) / 2;
        if a[mid] == x { ans = "Yes"; break }
        if x < a[mid] { right = mid - 1; }
        if a[mid] < x { left = mid + 1; }
    }

    println!("{}", ans);
    format!("{}", ans)
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 7;
        let x = 3;
        let a = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!("Yes", solve(n, x, a));
    }

    #[test]
    fn test_2() {
        let n = 7;
        let x = 9;
        let a = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!("No", solve(n, x, a));
    }

    #[test]
    fn test_3() {
        let n = 7;
        let x = 1;
        let a = vec![2, 3, 4, 5, 6, 7, 8];
        assert_eq!("No", solve(n, x, a));
    }
}
