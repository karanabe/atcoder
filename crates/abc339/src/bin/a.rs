// https://atcoder.jp/contests/abc339/tasks/abc339_a
// A - TLD
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let _: String = solve(&s);
}

fn solve(s: &str) -> String {
    let words: Vec<&str> = s.split('.').collect();
    let result = words.last().unwrap();

    println!("{result}");
    format!("{result}")
}

#[cfg(test)]
mod abc338 {
    use super::*;

    #[test]
    fn test_1() {
        let s: &str = "atcoder.jp";
        assert_eq!("jp", solve(s));
    }

    #[test]
    fn test_2() {
        let s: &str = "translate.google.com";
        assert_eq!("com", solve(s));
    }

    #[test]
    fn test_3() {
        let s: &str = ".z";
        assert_eq!("z", solve(s));
    }

    #[test]
    fn test_4() {
        let s: &str = "..........txt";
        assert_eq!("txt", solve(s));
    }

    #[test]
    fn test_5() {
        let s: &str = ".a";
        assert_eq!("a", solve(s));
    }
}
