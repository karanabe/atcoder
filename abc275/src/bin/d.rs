// https://atcoder.jp/contests/abc275/tasks/abc275_d
// D - Yet Another Recursive Function
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    let _: usize = solve(n);
}

fn solve(n: usize) -> usize {
    // let result: usize = fib(n);
    let mut memo: HashMap<usize, usize> = HashMap::new();
    let result: usize = fib_memo(n, &mut memo);
    println!("{}", result);
    result
}

#[allow(dead_code)]
#[inline(never)]
fn fib(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        fib(n.div_euclid(2)) + fib(n.div_euclid(3))
    }
}

fn fib_memo(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(&value) = memo.get(&n) {
        return value;
    }
    let result = match n {
        0 => 1,
        _ => fib_memo(n.div_euclid(2), memo) + fib_memo(n.div_euclid(3), memo),
    };
    memo.insert(n, result);

    result
}

#[cfg(test)]
mod abc275 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 2;
        assert_eq!(3, solve(n));
    }

    #[test]
    fn test_2() {
        let n: usize = 0;
        assert_eq!(1, solve(n));
    }

    #[test]
    fn test_3() {
        let n: usize = 100;
        assert_eq!(55, solve(n));
    }
}
