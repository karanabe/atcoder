// https://atcoder.jp/contests/abc355/tasks/abc355_a
// A - Who Ate the Cake?
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }
    let _: String = solve(a, b);
}

fn solve(a: usize, b: usize) -> String {
    let mut ans = 6;
    if a == b {
        println!("-1");
        return format!("-1");
    }

    ans = ans - a - b;

    println!("{ans}");
    format!("{ans}")
}

#[cfg(test)]
mod abc355 {
    use super::*;

    #[test]
    fn test_1() {
        let a = 1;
        let b = 2;
        assert_eq!("3", solve(a, b));
    }

    #[test]
    fn test_2() {
        let a = 1;
        let b = 1;
        assert_eq!("-1", solve(a, b));
    }

    #[test]
    fn test_3() {
        let a = 3;
        let b = 1;
        assert_eq!("2", solve(a, b));
    }

    #[test]
    fn test_4() {
        let a = 3;
        let b = 2;
        assert_eq!("1", solve(a, b));
    }
}
