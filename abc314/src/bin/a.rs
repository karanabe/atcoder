// https://atcoder.jp/contests/abc314/tasks/abc314_a
// A - 3.14
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: &str = solve(n);
}

fn solve(n: usize) -> &'static str {
    let pi: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    let result: &str = &pi[0..n+2];
    println!("{}", result);
    result
}



#[cfg(test)]
mod abc314 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 2;
        assert_eq!("3.14", solve(n));
    }

    #[test]
    fn test_2() {
        let n: usize = 32;
        assert_eq!("3.14159265358979323846264338327950", solve(n));
    }

    #[test]
    fn test_3() {
        let n: usize = 100;
        assert_eq!("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679", solve(n));
    }
}
