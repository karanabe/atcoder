// https://atcoder.jp/contests/abc339/tasks/abc339_c
// C - Perfect Bus
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let _: String = solve(n, a);
}

fn solve(n: usize, a: Vec<i64>) -> String {
    let mut first: i64 = 0;

    let mut current: i64 = 0;
    let mut result: i64 = 0;

    for i in 0..n {
        current = current + a[i];
        result += a[i];
        if current < 0 {
            first += current.abs();
            current = 0;
        }
    }

    result += first;

    // println!("{} {}", first, result);
    println!("{result}");
    format!("{result}")
}

#[cfg(test)]
mod abc339 {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let a: Vec<i64> = vec![3, -5, 7, -4];
        assert_eq!("3", solve(n, a));
    }

    #[test]
    fn test_2() {
        let n = 5;
        let a: Vec<i64> = vec![0, 0, 0, 0, 0];
        assert_eq!("0", solve(n, a));
    }

    #[test]
    fn test_3() {
        let n = 4;
        let a: Vec<i64> = vec![-1, 1000000000, 1000000000, 1000000000];
        assert_eq!("3000000000", solve(n, a));
    }
}
