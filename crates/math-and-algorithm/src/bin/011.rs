// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k
// 011 - Print Prime Numbers
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: usize = solve(n,);
}

fn solve(n: usize) -> usize {
    for i in 2..=n {
        if is_prime(i) {
            print!("{i} ");
        }
    }
    println!();
    0
}

fn is_prime(i: usize) -> bool {
    for j in 2..=i {
        if j != i && i % j == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        assert_eq!(120, solve(n));
    }
}
