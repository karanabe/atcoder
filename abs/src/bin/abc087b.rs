// https://atcoder.jp/contests/abs/tasks/abc087_b
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let _: usize = solve(a, b, c, x);
}

fn solve(a: usize, b: usize, c: usize, x: usize) -> usize {
    let mut calc_count: usize = 0;
    for c500 in 0..=a {
        for c100 in 0..=b {
            for c50 in 0..=c {
                if (c500 * 500 + c100 * 100 + c50 * 50) == x { calc_count += 1; }
            }
        }
    }
    print!("{}", calc_count);
    calc_count
}


#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn calc_count_test_1() {
        let a: usize = 2;
        let b: usize = 2;
        let c: usize = 2;
        let x: usize = 100;
        assert_eq!(2, solve(a, b, c, x));
    }

    #[test]
    fn calc_count_test_2() {
        let a: usize = 5;
        let b: usize = 1;
        let c: usize = 0;
        let x: usize = 150;
        assert_eq!(0, solve(a, b, c, x));
    }

    #[test]
    fn calc_count_test_3() {
        let a: usize = 30;
        let b: usize = 40;
        let c: usize = 50;
        let x: usize = 6000;
        assert_eq!(213, solve(a, b, c, x));
    }
}
