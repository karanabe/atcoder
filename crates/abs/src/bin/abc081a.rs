// https://atcoder.jp/contests/abs/tasks/abc081_a
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let _: usize = solve(s);
}

fn solve(s: String) -> usize {
    let mut marble_count: usize = 0;
    for ch in s.chars() {
        marble_count += if ch == '1' { 1 } else { 0 };
    }
    print!("{}", marble_count);
    marble_count
}

#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn marble_count_101() {
        let s: String = "101".to_string();
        assert_eq!(2, solve(s));
    }

    #[test]
    fn marble_count_000() {
        let s: String = "000".to_string();
        assert_eq!(0, solve(s));
    }

    #[test]
    fn marble_count_110() {
        let s: String = "110".to_string();
        assert_eq!(2, solve(s));
    }

    #[test]
    fn marble_count_111() {
        let s: String = "111".to_string();
        assert_eq!(3, solve(s));
    }

}
