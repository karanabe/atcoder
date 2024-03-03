// https://atcoder.jp/contests/abc340/tasks/abc341_a
// A - Print 341
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: String = solve(n);
}

fn solve(n: usize) -> String {
    let mut result = "1".to_string();
    for _i in 0..n {
        result = format!("{}01", result);
    }
    println!("{result}");
    format!("{result}")
}

#[cfg(test)]
mod abc341 {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        assert_eq!("101010101", solve(n));
    }

    #[test]
    fn test_2() {
        let n = 1;
        assert_eq!("101", solve(n));
    }
    #[test]
    fn test_3() {
        let n = 10;
        assert_eq!("101010101010101010101", solve(n));
    }
}
