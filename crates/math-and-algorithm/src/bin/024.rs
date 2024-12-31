// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_x
// 024 - Answer Exam Randomly
use proconio::input;

fn main() {
    input! {
        n: usize,
        pq: [[usize; 2]; n],
    }
    let _: f64 = solve(n, pq);
}

fn solve(_n: usize, pq: Vec<Vec<usize>>) -> f64 {
    let mut result: f64 = 0.0;
    for arr in pq.iter() {
        result += arr[1] as f64 / arr[0] as f64;
    }
    println!("{result:.6}");
    result
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let pq: Vec<Vec<usize>> = vec![vec![2, 50], vec![4, 100]];
        assert_eq!(50.0, solve(n, pq));
    }
}
