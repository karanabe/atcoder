// https://atcoder.jp/contests/abc340/tasks/abc340_a
// A - Arithmetic Progression
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    }
    let _: String = solve(a, b, d);
}

fn solve(a: usize, b: usize, d: usize) -> String {
    let mut result: Vec<String> = vec![];
    for i in (a..=b).step_by(d) {
        result.push(i.to_string());
    }

    let ans = result.join(" ");
    println!("{ans}");
    format!("{ans}")
}

#[cfg(test)]
mod abc340 {
    use super::*;

    #[test]
    fn test_1() {
        let a = 3;
        let b = 9;
        let d = 2;
        assert_eq!("3 5 7 9", solve(a, b, d));
    }

    #[test]
    fn test_2() {
        let a = 10;
        let b = 10;
        let d = 1;
        assert_eq!("10", solve(a, b, d));
    }
}
