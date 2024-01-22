// https://atcoder.jp/contests/adt_all_20240118_3/tasks/abc276_a
// B - Rightmost
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let _: String = solve(&s);
}

fn solve(s: &str) -> String {
    let mut result = -1;
    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            result = i as i32 + 1;
        }
    }
    println!("{result}");
    format!("{result}")
}

#[cfg(test)]
mod adt_20240118_03 {
    use super::*;

    #[test]
    fn test_1() {
        let s: &str = "abcdaxayz";
        assert_eq!("7", solve(s));
    }

    #[test]
    fn test_2() {
        let s: &str = "bcbbbz";
        assert_eq!("-1", solve(s));
    }

    #[test]
    fn test_3() {
        let s: &str = "aaaaa";
        assert_eq!("5", solve(s));
    }

    #[test]
    fn test_4() {
        let s: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!("100", solve(s));
    }
}
