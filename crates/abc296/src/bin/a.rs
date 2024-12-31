// https://atcoder.jp/contests/abc296/tasks/abc296_a
// A - Alternately
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let _: &str = solve(n, s);
}

fn solve(n: usize, s: Vec<char>) -> &'static str {
    let mut result = "Yes";
    for i in 1..n {
        if s[i - 1] == s[i] {
            result = "No";
            println!("{}", result);
            return result;
        }
    }
    println!("{}", result);
    result
}

#[cfg(test)]
mod abc296 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 6;
        let s: Vec<char> = "MFMFMF".to_string().chars().collect();
        assert_eq!("Yes", solve(n, s));
    }

    #[test]
    fn test_2() {
        let n: usize = 9;
        let s: Vec<char> = "FMFMMFMFM".to_string().chars().collect();
        assert_eq!("No", solve(n, s));
    }

    #[test]
    fn test_3() {
        let n: usize = 1;
        let s: Vec<char> = "F".to_string().chars().collect();
        assert_eq!("Yes", solve(n, s));
    }
}
