// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_r
// 018 - Convenience Store 1
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(n, a);
}

fn solve(_n: usize, a: Vec<usize>) -> usize {
    let c100 = a.iter().filter(|&x| *x == 100).count();
    let c200 = a.iter().filter(|&x| *x == 200).count();
    let c300 = a.iter().filter(|&x| *x == 300).count();
    let c400 = a.iter().filter(|&x| *x == 400).count();

    let result = (c100 * c400) + (c200 * c300);
    println!("{result}");
    result
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        let a = vec![100, 300, 400, 400, 200];
        assert_eq!(3, solve(n, a));
    }
}
