// https://atcoder.jp/contests/abc296/tasks/abc296_b
// B - Chessboard
use proconio::input;

fn main() {
    input! {
        mut s: [String; 8],
    }
    let _: String = solve(&mut s);
}

fn solve(s: &mut [String]) -> String {
    let mut result: String = "".to_string();
    s.reverse();
    for (i, line) in s.iter().enumerate() {
        let col: usize = line.chars().position(|x| x == '*').unwrap_or(99);
        if col < 8 {
            result = format!("{}{}", std::char::from_u32(col as u32 + 97).unwrap(), i + 1);
            println!("{}", result);
        }
    }
    result
}

#[cfg(test)]
mod abc296 {
    use super::*;

    #[test]
    fn test_1() {
        let mut s: Vec<String> = vec![
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "*.......".to_string(),
        ];
        assert_eq!("a1", solve(&mut s));
    }

    #[test]
    fn test_2() {
        let mut s: Vec<String> = vec![
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            ".*......".to_string(),
            "........".to_string(),
            "........".to_string(),
        ];
        assert_eq!("b3", solve(&mut s));
    }

    #[test]
    fn test_3() {
        let mut s: Vec<String> = vec![
            ".......*".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
        ];
        assert_eq!("h8", solve(&mut s));
    }
}
