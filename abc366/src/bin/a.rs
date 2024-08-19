//
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }
    let _: String = solve(n, t, a);
}

fn solve(n: usize, t: usize, a: usize) -> String {
    if t.abs_diff(a) < n - (t + a) {
        println!("No");
        return format!("No");
    }

    println!("Yes");
    format!("Yes")
}

#[cfg(test)]
mod abc355 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("Yes", solve(7, 4, 2));
    }

    #[test]
    fn test_2() {
        assert_eq!("No", solve(99, 12, 48));
    }

    #[test]
    fn test_3() {
        assert_eq!("No", solve(1, 0, 0));
    }
}
