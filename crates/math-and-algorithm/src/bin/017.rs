// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_q
// 017 - Least Common Multiple of N Integers
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(n, a);
}

fn solve(_n: usize, a: Vec<usize>) -> usize {
    // The initial value of fold is set to 1 in order not to affect the determination of the least common multiple.
    let result = a.iter().fold(1, |acc, x| lcm(acc, *x));
    println!("{result}");
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
fn gcd_not_good(aa: usize, bb: usize) -> usize {
    let mut a = aa;
    let mut b = bb;
    while a >= 1 && b >= 1 {
        if a < b {
            b = b % a;
        } else {
            a = a % b;
        }
    }
    if a >= 1 { return a; }
    b
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let a = vec![12, 18, 14];
        assert_eq!(252, solve(n, a));
    }
}
