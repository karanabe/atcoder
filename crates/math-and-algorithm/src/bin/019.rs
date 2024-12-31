// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_s
// 019 - Choose Cards 1
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(n, a);
}

fn solve(_n: usize, a: Vec<usize>) -> usize {
    let red = a.iter().filter(|&x| *x == 1).count();
    let yellow = a.iter().filter(|&x| *x == 2).count();
    let blue = a.iter().filter(|&x| *x == 3).count();

    let result = red * (red - 1) / 2 + yellow * (yellow - 1) / 2 + blue * (blue - 1) / 2;
    println!("{result}");
    result
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 6;
        let a = vec![1, 3, 2, 1, 1, 2];
        assert_eq!(4, solve(n, a));
    }
}
