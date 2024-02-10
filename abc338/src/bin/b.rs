// https://atcoder.jp/contests/abc335/tasks/abc338_b
// B - Frequency
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }
    let _: String = solve(&s);
}

fn solve(s: &str) -> String {
    let mut counts = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut pairs: Vec<(&char, &i32)> = counts.iter().collect();

    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    println!("{}", pairs[0].0);
    format!("{}", pairs[0].0)
}

#[cfg(test)]
mod abc338 {
    use super::*;

    #[test]
    fn test_1() {
        let s: &str = "frequency";
        assert_eq!("e", solve(s));
    }

    #[test]
    fn test_2() {
        let s: &str = "atcoder";
        assert_eq!("a", solve(s));
    }

    #[test]
    fn test_3() {
        let s: &str = "pseudopseudohypoparathyroidism";
        assert_eq!("o", solve(s));
    }

    #[test]
    fn test_4() {
        let s: &str = "cocoa";
        assert_eq!("c", solve(s));
    }

    #[test]
    fn test_5() {
        let s: &str = "banana";
        assert_eq!("a", solve(s));
    }
}
