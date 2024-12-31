// https://atcoder.jp/contests/adt_all_20240118_3/tasks/abc244_a
// A - Last Letter
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let _: String = solve(n, &s);
}

fn solve(n: usize, s: &str) -> String {
    println!("{}", &s[n-1..n]);
    format!("{}", &s[n-1..n])
}

#[cfg(test)]
mod adt_20240118_03 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 5;
        let s: &str = "abcde";
        assert_eq!("e", solve(n, s));
    }

    #[test]
    fn test_2() {
        let n: usize = 1;
        let s: &str = "a";
        assert_eq!("a", solve(n, s));
    }
}
