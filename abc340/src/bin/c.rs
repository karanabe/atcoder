// https://atcoder.jp/contests/abc340/tasks/abc340_c
// C - Divide and Divide
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i64,
    }
    let _: String = solve(n);
}

fn solve(n: i64) -> String {
    let mut memo: HashMap<i64, i64> = HashMap::new();
    let ans: i64 = calc(n, &mut memo);
    println!("{ans}");
    format!("{ans}")
}

fn calc(n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n <= 1 {
        return 0;
    }

    if let Some(&value) = memo.get(&n) {
        return value;
    }

    let q = n / 2;
    let r = (n + 1) / 2;

    let result = n + calc(q, memo) + calc(r, memo);

    memo.insert(n, result);

    result
}

#[cfg(test)]
mod abc340 {
    use super::*;

    #[test]
    fn test_1() {
        let n: i64 = 3;
        assert_eq!("5", solve(n));
    }

    #[test]
    fn test_2() {
        let n: i64 = 340;
        assert_eq!("2888", solve(n));
    }

    #[test]
    fn test_3() {
        let n: i64 = 100000000000000000;
        assert_eq!("5655884811924144128", solve(n));
    }
}
