// https://atcoder.jp/contests/abc335/tasks/arc170_a
// A - Yet Another AB Problem
use proconio::input;

fn main() {
    input! {
        n: usize,
        s1: String,
        s2: String,
    }
    let _: String = solve(n, s1, s2);
}

fn solve(_n: usize, s1: String, s2: String) -> String {
    let result = remove_matches(&s1, &s2);
    println!("{result}");
    result
}

fn remove_matches(s1: &str, s2: &str) -> String {
    #[allow(unused_assignments)]
    let mut result = "-1".to_string();
    let mut flag_a = false;
    let mut flag_b = false;
    let mut ans = 0;
    let mut acc = 0;

    let len = s1.len();

    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    for _ in 0..len {

        let c1 = iter1.next().unwrap();
        let c2 = iter2.next().unwrap();

        // s=A, t=A or s=B, t=B
        if c1 == c2 {
            if c1 == 'A' {
                flag_a = true;
            } else {
                flag_b = true;
            }
        }
        // s=B, t=A
        else if c2 == 'A' {
            flag_a = true;
            flag_b = false;
            ans += 1;
            acc += 1;
        }
        // s=A, t=B
        else if flag_a {
            flag_b = true;
            if acc == 0 {
                ans += 1;
            } else {
                acc -= 1;
            }
        }
        // never come t=A
        else {
            result = "-1".to_string();
            return result;
        }
    }
    if !flag_b {
        result = "-1".to_string();
    } else {
        result = ans.to_string();
    }
    result
}

#[cfg(test)]
mod arc170 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 5;
        let s1: String = "BAABA".to_string();
        let s2: String = "AABAB".to_string();
        assert_eq!("2", solve(n, s1, s2));
    }

    #[test]
    fn test_2() {
        let n: usize = 2;
        let s1: String = "AB".to_string();
        let s2: String = "BA".to_string();
        assert_eq!("-1", solve(n, s1, s2));
    }

    #[test]
    fn test_3() {
        let n: usize = 10;
        let s1: String = "AABAABABAB".to_string();
        let s2: String = "AAABABABAB".to_string();
        assert_eq!("1", solve(n, s1, s2));
    }

    #[test]
    fn test_4() {
        let n: usize = 10;
        let s1: String = "AABAABABAB".to_string();
        let s2: String = "AABAABABAB".to_string();
        assert_eq!("0", solve(n, s1, s2));
    }

    #[test]
    fn test_5() {
        let n: usize = 10;
        let s1: String = "BABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABA".to_string();
        let s2: String = "ABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABABAB".to_string();
        assert_eq!("100", solve(n, s1, s2));
    }

    #[test]
    fn test_6() {
        let n: usize = 4;
        let s1: String = "BAAB".to_string();
        let s2: String = "ABBA".to_string();
        assert_eq!("-1", solve(n, s1, s2));
    }

    #[test]
    fn test_7() {
        let n: usize = 4;
        let s1: String = "BBAA".to_string();
        let s2: String = "AABB".to_string();
        assert_eq!("2", solve(n, s1, s2));
    }

    #[test]
    fn test_8() {
        let n: usize = 10;
        let s1: String = "AABAABABAB".to_string();
        let s2: String = "AABBAAABAB".to_string();
        assert_eq!("2", solve(n, s1, s2));
    }

    #[test]
    fn test_9() {
        let n: usize = 10;
        let s1: String = "AABAABABAB".to_string();
        let s2: String = "BABAABABAB".to_string();
        assert_eq!("-1", solve(n, s1, s2));
    }

    #[test]
    fn test_10() {
        let n: usize = 2;
        let s1: String = "BA".to_string();
        let s2: String = "AA".to_string();
        assert_eq!("-1", solve(n, s1, s2));
    }

    #[test]
    fn test_11() {
        let n: usize = 2;
        let s1: String = "BA".to_string();
        let s2: String = "AA".to_string();
        assert_eq!("-1", solve(n, s1, s2));
    }
}
