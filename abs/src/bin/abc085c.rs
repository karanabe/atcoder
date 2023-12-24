// https://atcoder.jp/contests/abs/tasks/abc088_b
use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
    }
    let _: String = solve(n, y);
}

fn solve(n: usize, y: usize) -> String {
    for s10000 in 0..=n {
        for s5000 in 0..=n - s10000 {
            let s1000 = n - s10000 - s5000;
            if y == (s10000*10000 + s5000*5000 + s1000*1000) {
                print!("{} {} {}", s10000, s5000, s1000);
                let result = s10000*10000 + s5000*5000 + s1000*1000;
                return result.to_string();
            }
        }
    }
    print!("-1 -1 -1");
    return "-1 -1 -1".to_string();
}


#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 9;
        let y: usize = 45000;
        assert_eq!("45000", solve(n, y));
    }

    #[test]
    fn test_2() {
        let n: usize = 20;
        let y: usize = 196000;
        assert_eq!("-1 -1 -1", solve(n, y));
    }

    #[test]
    fn test_3() {
        let n: usize = 1000;
        let y: usize = 1234000;
        assert_eq!("1234000", solve(n, y));
    }

    #[test]
    fn test_4() {
        let n: usize = 2000;
        let y: usize = 20000000;
        assert_eq!("20000000", solve(n, y));
    }
}
