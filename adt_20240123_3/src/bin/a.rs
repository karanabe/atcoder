// https://atcoder.jp/contests/adt_easy_20240123_3/tasks/abc259_a
// A - Growth Record
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize,
    }
    let _: String = solve(n, m, x, t, d);
}

fn solve(_n: usize, m: usize, x: usize, t: usize, d: usize) -> String {
    let result = if m < x {
        t - ((x - m) * d)
    } else {
        t
    };

    println!("{result}");
    format!("{result}")
}


#[cfg(test)]
mod adt_20240123_03 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("168", solve(38, 20, 17, 168, 3));
    }

    #[test]
    fn test_2() {
        assert_eq!("1", solve(1, 0, 1, 3, 2));
    }

    #[test]
    fn test_3() {
        assert_eq!("90", solve(100, 10, 100, 180, 1));
    }
}
