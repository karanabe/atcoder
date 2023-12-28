// https://atcoder.jp/contests/abc275/tasks/abc275_b
// B - ABC-DEF
use proconio::input;

fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
        d: u128,
        e: u128,
        f: u128,
    }
    let _: u128 = solve(a, b, c, d, e, f);
}

fn solve(a: u128, b: u128, c: u128, d: u128, e: u128, f: u128) -> u128 {
    let m = 998244353;
    let abc = a % m * b % m * c % m;
    let def = d % m * e % m * f % m;
    let result = (abc - def + m) % m;
    println!("{}", result);
    return result;
}

#[cfg(test)]
mod abc275 {
    use super::*;

    #[test]
    fn test_1() {
        let a: u128 = 2;
        let b: u128 = 3;
        let c: u128 = 5;
        let d: u128 = 1;
        let e: u128 = 2;
        let f: u128 = 4;
        assert_eq!(22, solve(a, b, c, d, e, f));
    }

    #[test]
    fn test_2() {
        let a: u128 = 1;
        let b: u128 = 1;
        let c: u128 = 1000000000;
        let d: u128 = 0;
        let e: u128 = 0;
        let f: u128 = 0;
        assert_eq!(1755647, solve(a, b, c, d, e, f));
    }

    #[test]
    fn test_3() {
        let a: u128 = 1000000000000000000;
        let b: u128 = 1000000000000000000;
        let c: u128 = 1000000000000000000;
        let d: u128 = 1000000000000000000;
        let e: u128 = 1000000000000000000;
        let f: u128 = 1000000000000000000;
        assert_eq!(0, solve(a, b, c, d, e, f));
    }
}
