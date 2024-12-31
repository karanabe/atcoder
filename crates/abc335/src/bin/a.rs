// https://atcoder.jp/contests/abc335/tasks/abc335_a
// A - 202<s>3</s>
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    let _: String = solve(s);
}

fn solve(mut s: Vec<char>) -> String {
    if let Some(last) = s.last_mut() {
        *last = '4';
    }
    let result = s.iter().collect::<String>();
    println!("{}", result);
    result
}

#[cfg(test)]
mod abc335 {
    use super::*;

    #[test]
    fn test_1() {
        let s: Vec<char> = "hello2023".to_string().chars().collect();
        assert_eq!("hello2024", solve(s));
    }

    #[test]
    fn test_2() {
        let s: Vec<char> = "worldtourfinals2023".to_string().chars().collect();
        assert_eq!("worldtourfinals2024", solve(s));
    }

    #[test]
    fn test_3() {
        let s: Vec<char> = "2023".to_string().chars().collect();
        assert_eq!("2024", solve(s));
    }

    #[test]
    fn test_4() {
        let s: Vec<char> = "20232023".to_string().chars().collect();
        assert_eq!("20232024", solve(s));
    }
}
