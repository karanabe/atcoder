// https://atcoder.jp/contests/abc335/tasks/abc336_c
// C - Even Digits
use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let _: String = solve(n);
}

fn solve(mut n: usize) -> String {
    if n == 1 {
        println!("0");
        return "0".to_string();
    }
    let mut v = vec![];
    n -= 1;
    while n > 0 {
        v.push((n%5) * 2);
        n /= 5;
    }
    v.reverse();
    for i in v.clone() {
        print!("{i}");
    }
    println!("");
    let s: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    s.join("")
}


#[cfg(test)]
mod abc336 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 8;
        assert_eq!("24", solve(n));
    }

    #[test]
    fn test_2() {
        let n: usize = 133;
        assert_eq!("2024", solve(n));
    }

    #[test]
    fn test_3() {
        let n: usize = 31415926535;
        assert_eq!("2006628868244228", solve(n));
    }

    #[test]
    fn test_4() {
        let n: usize = 1;
        assert_eq!("0", solve(n));
    }
}
