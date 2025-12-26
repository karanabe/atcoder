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
    let result = if s.len() != 8 {
        "No"
    } else {
        let first = s.chars().next().unwrap();
        let last = s.chars().last().unwrap();
        let middle = &s[1..s.len() - 1];

        let is_uppercase = first.is_ascii_uppercase() && last.is_ascii_uppercase();

        let number = middle.parse::<u32>().ok();
        let is_number = number.is_some();
        let is_in_range = number.is_some_and(|num| (100000..=999999).contains(&num));

        if is_uppercase && is_number && is_in_range {
            "Yes"
        } else {
            "No"
        }
    };

    println!("{result}");
    result.to_string()
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
