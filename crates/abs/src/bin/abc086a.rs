// https://atcoder.jp/contests/abs/tasks/abc086_a
use proconio::input;

fn main() {
    input! {
        a: usize, b: usize,
    }

    let _: &str = solve(a, b);

}

fn solve(a: usize, b: usize) -> &'static str {
    let calc_result = a * b;
    if calc_result % 2 == 0 {
        print!("Even");
        return "Even";
    } else {
        print!("Odd");
        return "Odd";
    }
}

#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn even_1() {
        let a = 2;
        let b = 1234;
        assert_eq!("Even", solve(a, b));
    }

    #[test]
    fn even_2() {
        let a = 82;
        let b = 10000;
        assert_eq!("Even", solve(a, b));
    }

    #[test]
    fn even_3() {
        let a = 12;
        let b = 345;
        assert_eq!("Even", solve(a, b));
    }

    #[test]
    fn odd_1() {
        let a = 11;
        let b = 13;
        assert_eq!("Odd", solve(a, b));
    }

    #[test]
    fn odd_2() {
        let a = 1;
        let b = 1;
        assert_eq!("Odd", solve(a, b));
    }

    #[test]
    fn odd_3() {
        let a = 19;
        let b = 9999;
        assert_eq!("Odd", solve(a, b));
    }
}
