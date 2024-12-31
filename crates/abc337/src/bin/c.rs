// https://atcoder.jp/contests/abc335/tasks/abc337_c
//
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let _: String = solve(n, a);
}

fn solve(n: usize, a: Vec<i32>) -> String {
    let mut current = -1;
    #[allow(unused_assignments)]
    let mut num: i32 = 0;
    let mut result: String = "".to_string();
    let mut hashmap: HashMap<i32, usize> = HashMap::new();

    for (index, element) in a.iter().enumerate() {
        hashmap.insert(*element, index);
    }

    for _i in 0..n {
        num = *hashmap.get(&current).unwrap() as i32 + 1;
        print!("{} ", num);
        // Below is reason for TLE. Just for testing.
        // result = format!("{result}{num} ");
        current = num ;
    }
    println!();
    // result.trim_start().trim_end().to_string()
    "test".to_string()
}

#[cfg(test)]
mod abc337 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 6;
        let a: Vec<i32> = vec![4, 1, -1, 5, 3, 2];
        assert_eq!("3 5 4 1 2 6", solve(n, a));
    }

    #[test]
    fn test_2() {
        let n: usize = 10;
        let a: Vec<i32> = vec![-1, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!("1 2 3 4 5 6 7 8 9 10", solve(n, a));
    }

    #[test]
    fn test_3() {
        let n: usize = 30;
        let a: Vec<i32> = vec![3, 25, 20, 6, 18, 12, 26, 1, 29, -1, 21, 17, 23, 9, 8, 30, 10, 15, 22, 27, 4, 13, 5, 11, 16, 24, 28, 2, 19, 7];
        assert_eq!("10 17 12 6 4 21 11 24 26 7 30 16 25 2 28 27 20 3 1 8 15 18 5 23 13 22 19 29 9 14", solve(n, a));
    }
}
