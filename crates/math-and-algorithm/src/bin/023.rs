// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_w
// 023 - Dice Expectation
use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [f64; n],
        r: [f64; n],
    }
    let _: f64 = solve(n, b, r);
}

fn solve(n: usize, b: Vec<f64>, r: Vec<f64>) -> f64 {
    let len = n as f64;
    let result: f64 = (b.iter().sum::<f64>() / len) + (r.iter().sum::<f64>() / len);
    println!("{result:.6}");
    result
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let b: Vec<f64> = vec![1.0, 2.0, 3.0];
        let r: Vec<f64> = vec![10.0, 20.0, 30.0];
        assert_eq!(22.0, solve(n, b, r));
    }
}
