// https://atcoder.jp/contests/abc352/tasks/abc352_a
// A - AtCoder Line
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
    }
    let _: String = solve(n, x, y, z);
}

fn solve(_n: usize, x: usize, y: usize, z: usize) -> String {
    let lower = x.min(y);
    let upper = x.max(y);

    let ans = if lower < z && z < upper { "Yes" } else { "No" };

    println!("{ans}");
    format!("{ans}")
}

#[cfg(test)]
mod abc352 {
    use super::*;

    #[test]
    fn test_1() {
        let n = 10;
        let x = 3;
        let y = 2;
        let z = 9;
        assert_eq!("No", solve(n, x, y, z));
    }

    #[test]
    fn test_2() {
        let n = 7;
        let x = 6;
        let y = 1;
        let z = 3;
        assert_eq!("Yes", solve(n, x, y, z));
    }
    
    #[test]
    fn test_3() {
        let n = 100;
        let x = 23;
        let y = 67;
        let z = 45;
        assert_eq!("Yes", solve(n, x, y, z));
    }
}
