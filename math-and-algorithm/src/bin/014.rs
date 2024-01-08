// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_n
// 014 - Factorization
use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let _: String = solve(n);
}

fn solve(n: f64) -> String {
    let mut result: Vec<usize> = Vec::new();
    let mut tmp = n as usize;
    for i in 2..=n.sqrt().floor() as usize {
        while tmp % i == 0 {
            result.push(i);
            tmp /= i;
        }
    }
    if tmp > 1 {
        result.push(tmp);
    }
    let r = format!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    println!("{}", r);
    return r;
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 10.0;
        assert_eq!("2 5", solve(n));
    }

    #[test]
    fn test_2() {
        let n = 36.0;
        assert_eq!("2 2 3 3", solve(n));
    }
}
