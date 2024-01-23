// https://atcoder.jp/contests/adt_easy_20240123_3/tasks/abc266_b
// D - Modulo Number
use proconio::input;

fn main() {
    input! {
        n: i64
    }
    let _: String = solve(n);
}

fn solve(n: i64) -> String {
    let m =  998244353;
    let result = (n % m + m) % m;
    // let result = n.rem_euclid(m);
    println!("{result}");
    format!("{result}")
}

#[cfg(test)]
mod adt_20240123_03 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("1", solve(998244354));
    }

    #[test]
    fn test_2() {
        assert_eq!("998244349", solve(-9982443534));
    }

    #[test]
    fn test_3() {
        assert_eq!("0", solve(0));
    }
}
