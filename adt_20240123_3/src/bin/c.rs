// https://atcoder.jp/contests/adt_easy_20240123_3/tasks/abc281_b
// C - Sandwich Number
use proconio::input;

fn main() {
    input! {
        s: String
    }
    let _: String = solve(&s);
}

fn solve(s: &str) -> String {
    if s.len() != 8 {
        println!("No");
        return format!("No");
    }
    let first = s.chars().next().unwrap();
    let last = s.chars().last().unwrap();
    let middle = &s[1..s.len()-1];

    let is_uppercase = first.is_ascii_uppercase() && last.is_ascii_uppercase();

    let is_number = middle.parse::<u32>().is_ok();
    let is_in_range = if let Ok(num) = middle.parse::<u32>() {
        num >= 100000 && num <= 999999
    } else {
        false
    };

    if is_uppercase && is_number && is_in_range {
        println!("Yes");
        return format!("Yes");
    } else {
        println!("No");
        return format!("No");
    }
}


#[cfg(test)]
mod adt_20240123_03 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("Yes", solve("Q142857Z"));
    }

    #[test]
    fn test_2() {
        assert_eq!("No", solve("AB912278C"));
    }

    #[test]
    fn test_3() {
        assert_eq!("No", solve("X900000"));
    }

    #[test]
    fn test_4() {
        assert_eq!("No", solve("K012345K"));
    }

    #[test]
    fn test_5() {
        assert_eq!("No", solve("K"));
    }

    #[test]
    fn test_6() {
        assert_eq!("No", solve("K1K"));
    }
}
