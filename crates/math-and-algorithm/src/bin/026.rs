// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_z
// 026 - Coin Gacha
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: f64 = solve(n);
}

fn solve(n: usize) -> f64 {
    let mut calc: f64 = 0.0;
    for i in 0..n {
        calc += n as f64 / (n - i) as f64;
    }
    let result = format!("{calc:.12}");
    println!("{result}");
    result.parse::<f64>().unwrap()
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        assert_eq!(11.416666666667, solve(n));
    }
}
