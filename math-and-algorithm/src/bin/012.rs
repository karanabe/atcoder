// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_l
// 012 - Primality Test
use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let _: String = solve(n);
}

fn solve(n: f64) -> String {
    if is_prime(n) {
        println!("Yes");
        return "Yes".to_string();
    }
    println!("No");
    "No".to_string()
}

fn is_prime(i: f64) -> bool {
    for j in 2..=(i.sqrt() as usize) {
        if i as usize % j == 0 {
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
        let n: f64 = 53.0;
        assert_eq!("Yes", solve(n));
    }

    #[test]
    fn test_2() {
        let n: f64 = 77.0;
        assert_eq!("No", solve(n));
    }

    #[test]
    fn test_3() {
        let n: f64 = 472249589291.0;
        assert_eq!("Yes", solve(n));
    }

    #[test]
    fn test_4() {
        let n: f64 = 10000000000000.0;
        assert_eq!("No", solve(n));
    }
}
