// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_p
// 016 - Greatest Common Divisor of N Integers
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(n, a);
}

fn solve(n: usize, a: Vec<usize>) -> usize {
    let mut g = gcd(a[0], a[1]);
    for i in 2..n {
        g = gcd(g, a[i]);
    }
    println!("{g}");
    g
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
        let a = vec![12, 18, 24];
        assert_eq!(6, solve(n, a));
    }
}
