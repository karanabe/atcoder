// https://atcoder.jp/contests/abs/tasks/aarc065_c
use proconio::input;

fn main() {
    input! {
        s: String
    }
    let _: String = solve(s);
}

fn solve(s: String) -> String {
    let mut slice = &s[..];
    let words = ["dreamer", "eraser", "dream", "erase"];

    while !slice.is_empty() {
        let mut result = false;

        for word in words {
            if slice.ends_with(word) {
                slice = &slice[..(slice.len() - word.len())];
                result = true;
                break;
            }
        }

        if result == false { print!("NO"); return "NO".to_string(); }
    }
    print!("YES");
    return "YES".to_string();
}


#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn test_1() {
        let s: String = "erasedream".to_string();
        assert_eq!("YES", solve(s));
    }

    #[test]
    fn test_2() {
        let s: String = "dreameraser".to_string();
        assert_eq!("YES", solve(s));
    }

    #[test]
    fn test_3() {
        let s: String = "dreamerer".to_string();
        assert_eq!("NO", solve(s));
    }

    #[test]
    fn test_4() {
        let s: String = "dreamdreamdreamdreamdreamdreamdreamerdreamerdreamerdreamereraseerasedreamerdreamerdreamererasererasererasererasererasereraserdreamerdreamerdreamerdreamereraseerase".to_string();
        assert_eq!("YES", solve(s));
    }
}
