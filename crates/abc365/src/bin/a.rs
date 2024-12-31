//
use proconio::input;

fn main() {
    input! {
        y: usize
    }
    let _: String = solve(y);
}

fn solve(y: usize) -> String {
    let ans = if y % 4 != 0 {
        365
    } else if y % 100 != 0 {
        366
    } else if y % 400 != 0 {
        365
    } else {
        366
    };

    println!("{ans}");
    format!("{ans}")
}

#[cfg(test)]
mod abc355 {
    use super::*;

    #[test]
    fn test_1() {
        let a = 2023;
        assert_eq!("365", solve(a));
    }

    #[test]
    fn test_2() {
        let a = 1992;
        assert_eq!("366", solve(a));
    }

    #[test]
    fn test_3() {
        let a = 1800;
        assert_eq!("365", solve(a));
    }

    #[test]
    fn test_4() {
        let a = 1600;
        assert_eq!("366", solve(a));
    }
}
