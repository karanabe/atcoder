// https://atcoder.jp/contests/abs/tasks/abc085_b
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let _: usize = solve(&d);
}

fn solve(d: &[usize]) -> usize {
    let mut v: Vec<usize> = d.to_vec();
    v.sort();
    v.dedup();
    print!("{}", v.len());
    v.len()
}


#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn test_1() {
        let d: [usize; 4] = [10, 8, 8, 6];
        assert_eq!(3, solve(&d));
    }

    #[test]
    fn test_2() {
        let d: [usize; 3] = [15, 15, 15];
        assert_eq!(1, solve(&d));
    }

    #[test]
    fn test_3() {
        let d: [usize; 7] = [50, 30, 50, 100, 50, 80, 30];
        assert_eq!(4, solve(&d));
    }
}
