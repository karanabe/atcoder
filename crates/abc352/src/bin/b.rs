// https://atcoder.jp/contests/abc352/tasks/abc352_b
// B - Typing
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }
    let _: String = solve(&s, &t);
}

fn solve(s: &str, t: &str) -> String {
    let mut result = Vec::new();
    let mut t_chars = t.chars().enumerate();
    let s_chars = s.chars();

    for next_s in s_chars {
        for (i, next_t) in t_chars.by_ref() {
            if next_s == next_t {
                result.push(i + 1);
                break;
            }
        }
    }

    let ans = result
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{ans}");
    ans.to_string()
}

#[cfg(test)]
mod abc352 {
    use super::*;

    #[test]
    fn test_1() {
        let s: &str = "abc";
        let t: &str = "axbxyc";
        assert_eq!("1 3 6", solve(s, t));
    }

    #[test]
    fn test_2() {
        let s: &str = "aaaa";
        let t: &str = "bbbbaaaa";
        assert_eq!("5 6 7 8", solve(s, t));
    }

    #[test]
    fn test_3() {
        let s: &str = "atcoder";
        let t: &str = "atcoder";
        assert_eq!("1 2 3 4 5 6 7", solve(s, t));
    }
}
