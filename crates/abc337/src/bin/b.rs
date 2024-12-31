// https://atcoder.jp/contests/abc335/tasks/abc337_b
// B - Extended ABC
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let _: String = solve(s);
}

fn solve(s: String) -> String {
    let mut current_abc = 'A';
    for c in s.chars() {
        if c < current_abc {
            println!("No");
            return "No".to_string();
        }

        if c > current_abc {
            current_abc = c;
        }
    }
    println!("Yes");
    "Yes".to_string()
}

#[cfg(test)]
mod abc337 {
    use super::*;

    #[test]
    fn test_1() {
        let s: String = "AAABBBCCCCCCC".to_string();
        assert_eq!("Yes", solve(s));
    }

    #[test]
    fn test_2() {
        let s: String = "ACABABCBC".to_string();
        assert_eq!("No", solve(s));
    }

    #[test]
    fn test_3() {
        let s: String = "A".to_string();
        assert_eq!("Yes", solve(s));
    }

    #[test]
    fn test_4() {
        let s: String = "ABBBBBBBBBBBBBCCCCCC".to_string();
        assert_eq!("Yes", solve(s));
    }
}
