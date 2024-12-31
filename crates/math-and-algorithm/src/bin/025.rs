// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_y
// 025 - Jiro's Vacation
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let _: f64 = solve(n, a, b);
}

fn solve(_n: usize, a: Vec<usize>, b: Vec<usize>) -> f64 {
    let calc: f64 = a.iter().sum::<usize>() as f64 * 2.0 / 6.0 + b.iter().sum::<usize>() as f64 * 4.0 / 6.0;
    let result = format!("{calc:.12}");
    println!("{result}");
    result.parse::<f64>().unwrap()
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let a = vec![3, 1, 4, 1, 5];
        let b = vec![9, 2, 6, 5, 3];
        assert_eq!(21.333333333333, solve(n, a, b));
    }
}
