// https://atcoder.jp/contests/abc335/tasks/abc336_a
// A - Long Loong
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: String = solve(n);
}

fn solve(n: usize) -> String {
    let mut result = "L".to_string();
    print!("L");
    for _x in 0..n {
        print!("o");
        result += "o";
    }
    println!("ng");
    result += "ng";
    result
}

#[cfg(test)]
mod abc336 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 3;
        assert_eq!("Looong", solve(n));
    }

    #[test]
    fn test_2() {
        let n: usize = 1;
        assert_eq!("Long", solve(n));
    }

    #[test]
    fn test_3() {
        let n: usize = 5;
        assert_eq!("Looooong", solve(n));
    }
}
