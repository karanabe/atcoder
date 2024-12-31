// https://atcoder.jp/contests/abc335/tasks/abc338_a
// A - Capitalized?
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let _: String = solve(&s);
}

fn solve(s: &str) -> String {
    let n = s.len();
    let is_uppercase = s[0..1].chars().all(|c| c.is_uppercase());

    if n == 1 {
        if is_uppercase {
            println!("Yes");
            return format!("Yes");
        } else {
            println!("No");
            return format!("No");
        }
    }
    let is_lowercase = s[1..n].chars().all(|c| c.is_lowercase());

    if is_uppercase && is_lowercase {
        println!("Yes");
        return format!("Yes");
    }
    println!("No");
    format!("No")
}

#[cfg(test)]
mod abc338 {
    use super::*;

    #[test]
    fn test_1() {
        let s: &str = "Capitalized";
        assert_eq!("Yes", solve(s));
    }

    #[test]
    fn test_2() {
        let s: &str = "AtCoder";
        assert_eq!("No", solve(s));
    }

    #[test]
    fn test_3() {
        let s: &str = "yes";
        assert_eq!("No", solve(s));
    }

    #[test]
    fn test_4() {
        let s: &str = "A";
        assert_eq!("Yes", solve(s));
    }

    #[test]
    fn test_5() {
        let s: &str = "a";
        assert_eq!("No", solve(s));
    }
}
