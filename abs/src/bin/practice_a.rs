// https://atcoder.jp/contests/abs/tasks/practice_1
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize, c: usize,
        s: String,
    }

    let _: String = solve(a, b, c, s);
}

fn solve(a: usize, b: usize, c: usize, s: String) -> String {
    let result = format!("{} {}", (a+b+c), s);
    print!("{}", result);
    result
}

#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn practice_a() {
        let a = 1;
        let b = 2;
        let c = 3;
        let s: String = "test".to_string();
        assert_eq!("6 test", solve(a, b, c, s));
    }
}
