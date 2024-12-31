// https://atcoder.jp/contests/abc335/tasks/abc336_b
// B - CTZ
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: usize = solve(n);
}

fn solve(n: usize) -> usize {
    let binary = format!("{:b}", n);
    // println!("{}", binary);

    let mut count: usize = 0;
    for c in binary.chars().rev() {
        if c == '0' {
            count += 1;
        } else {
            break;
        }
    }
    println!("{count}");
    count
}

#[cfg(test)]
mod abc336 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 2024;
        assert_eq!(3, solve(n));
    }

    #[test]
    fn test_2() {
        let n: usize = 18;
        assert_eq!(1, solve(n));
    }

    #[test]
    fn test_3() {
        let n: usize = 5;
        assert_eq!(0, solve(n));
    }

    #[test]
    fn test_4() {
        let n: usize = 1000000;
        assert_eq!(6, solve(n));
    }
}
