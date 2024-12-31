// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_g
// 007 - Number of Multiples 1
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [usize; 2]
    }
    let _: usize = solve(n, xy);
}

fn solve(n: usize, xy: Vec<usize>) -> usize {
    let mut result = 0;
    for i in 1..=n {
        if i % xy[0] == 0 || i % xy[1] == 0 {
            result += 1;
        }
    }
    println!("{result}");
    result
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 15;
        let a = vec![3, 5];
        assert_eq!(7, solve(n, a));
    }

    #[test]
    fn test_2() {
        let n = 1000000;
        let a = vec![11, 13];
        assert_eq!(160839, solve(n, a));
    }
}
