// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i
// 009 - Brute Force 2
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let _: String = solve(n, s, a);
}

fn solve(n: usize, s: usize, a: Vec<usize>) -> String {
    // rotate n bit
    // this mean same as 2^n
    // example: 1 << 3
    // Note: 1 << 3 is 2^3 = 8
    // Note: left side is bit, right side is dec number
    // 0 = 0
    // 1 = 1
    // 10 = 2
    // 11 = 3
    // 100 = 4
    // 101 = 5
    // 110 = 6
    // 111 = 7
    for i in 0..(1 << n) {
        let mut result = 0;
        for (j, &x) in a.iter().enumerate() {
            // rotate j bit left to 1
            // if i is '110'
            // j = 1 and 2 is not 0
            // j = 010, 100 is not 0
            // j = 001 is 0 => (110 & 001)
            if i & (1 << j) != 0 {
                result += x;
            }
        }
        if result == s {
            println!("Yes");
            return "Yes".to_string();
        }
    }
    println!("No");
    "No".to_string()
}

#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let s = 11;
        let a = vec![2, 5, 9];
        assert_eq!("Yes", solve(n, s, a));
    }

    #[test]
    fn test_2() {
        let n = 4;
        let s = 11;
        let a = vec![3, 1, 4, 5];
        assert_eq!("No", solve(n, s, a));
    }

}
