// https://atcoder.jp/contests/abc275/tasks/abc275_a
// A - Find Takahashi
use proconio::input;

fn main() {
    input! {
        n: u8,
        h: [i32; n],
    }
    let _: usize = solve(&h);
}

fn solve(bridges: &[i32]) -> usize {
    let mut highest: i32 = 0;
    let mut result = 0;
    for (i, bridge) in bridges.iter().cloned().enumerate() {
        if highest < bridge {
            highest = bridge;
            result = i + 1;
        }
    }
    println!("{}", result);
    result
}

#[cfg(test)]
mod abc275 {
    use super::*;

    #[test]
    fn test_1() {
        let h: [i32; 3] = [50, 80, 70];
        assert_eq!(2, solve(&h));
    }

    #[test]
    fn test_2() {
        let h: [i32; 1] = [1000000000];
        assert_eq!(1, solve(&h));
    }

    #[test]
    fn test_3() {
        let h: [i32; 10] = [22, 75, 26, 45, 72, 81, 47, 29, 97, 2];
        assert_eq!(9, solve(&h));
    }
}
