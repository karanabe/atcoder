// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_u
// 021 - Combination Easy
use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let _: usize = solve(n, r);
}

fn solve(n: usize, r: usize) -> usize {
    let mut bottom = 1;
    let mut top = n;
    for i in 1..=r {
        bottom *= i;
        if i != r {
            top *= n-i;
        }
    }
    println!("{}", top / bottom);
    top / bottom
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 6;
        let r = 2;
        assert_eq!(15, solve(n, r));
    }
}
