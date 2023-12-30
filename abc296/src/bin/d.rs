// https://atcoder.jp/contests/abc296/tasks/abc296_d
// D - M<=ab
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let _: String = solve(n, m);
}

fn solve(n: usize, m: usize) -> String {
    if n * n < m {
        println!("-1");
        return "-1".to_string();
    }
    let mut result = n * n;
    for b in 1..=n {
        // Rounding up of m/b is the same as (m + b - 1)/b
        let a = (m + b - 1) / b;
        if a < b {
            break;
        }
        if n < a {
            continue;
        }
        if a * b < result {
            result = a * b;
        }
    }
    println!("{}", result);
    result.to_string()
}

#[cfg(test)]
mod abc314 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 5;
        let m: usize = 7;
        assert_eq!("8", solve(n, m));
    }

    #[test]
    fn test_2() {
        let n: usize = 2;
        let m: usize = 5;
        assert_eq!("-1", solve(n, m));
    }

    #[test]
    fn test_3() {
        let n: usize = 100000;
        let m: usize = 10000000000;
        assert_eq!("10000000000", solve(n, m));
    }
}
