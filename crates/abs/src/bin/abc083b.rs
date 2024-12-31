// https://atcoder.jp/contests/abs/tasks/abc083_b
use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }
    let _: u32 = solve(n, a, b);
}

fn solve(n: u32, a: u32, b: u32) -> u32 {
    let mut number_vec: Vec<u32> = Vec::new();
    for num in 0..=n {
        let mut sum_digit: u32 = 0;
        for i in num.to_string().chars() {
            sum_digit += i.to_digit(10).unwrap();
        }
        if a <= sum_digit && sum_digit <= b {
            number_vec.push(num);
        }
    }
    let result: u32 = number_vec.iter().sum();
    print!("{}", result);
    result
}


#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn calc_sums_test_1() {
        let n: u32 = 20;
        let a: u32 = 2;
        let b: u32 = 5;
        assert_eq!(84, solve(n, a, b));
    }

    #[test]
    fn calc_sums_test_2() {
        let n: u32 = 10;
        let a: u32 = 1;
        let b: u32 = 2;
        assert_eq!(13, solve(n, a, b));
    }

    #[test]
    fn calc_sums_test_3() {
        let n: u32 = 100;
        let a: u32 = 4;
        let b: u32 = 16;
        assert_eq!(4554, solve(n, a, b));
    }

}
