use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![[-1i64 << 60; 3]; n + 1];
    dp[0] = [0; 3];

    for i in 0..n {
        if s[i] == 'R' {
            dp[i + 1][1] = dp[i][0].max(dp[i][2]) + 1;
            dp[i + 1][0] = dp[i][1].max(dp[i][2]);
        } else if s[i] == 'P' {
            dp[i + 1][2] = dp[i][0].max(dp[i][1]) + 1;
            dp[i + 1][1] = dp[i][0].max(dp[i][2]);
        } else {
            dp[i + 1][0] = dp[i][1].max(dp[i][2]) + 1;
            dp[i + 1][2] = dp[i][0].max(dp[i][1]);
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]).max(dp[n][2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 6;
        let s = "PRSSRS".chars().collect();
        assert_eq!(calculate_max_wins(n, &s), 5);
    }

    #[test]
    fn test_case_2() {
        let n = 10;
        let s = "SSSSSSSSSS".chars().collect();
        assert_eq!(calculate_max_wins(n, &s), 5);
    }

    #[test]
    fn test_case_3() {
        let n = 24;
        let s = "SPRPSRRRRRPPRPRPSSRSPRSS".chars().collect();
        assert_eq!(calculate_max_wins(n, &s), 18);
    }

    #[test]
    fn test_case_4() {
        let n = 1;
        let s = "R".chars().collect();
        assert_eq!(calculate_max_wins(n, &s), 1);
    }

    #[test]
    fn test_case_5() {
        let n = 2;
        let s = "RS".chars().collect();
        assert_eq!(calculate_max_wins(n, &s), 2);
    }
}

fn calculate_max_wins(n: usize, s: &Vec<char>) -> i64 {
    let mut dp = vec![[-1i64 << 60; 3]; n + 1];
    dp[0] = [0; 3];

    for i in 0..n {
        if s[i] == 'R' {
            dp[i + 1][1] = dp[i][0].max(dp[i][2]) + 1;
            dp[i + 1][0] = dp[i][1].max(dp[i][2]);
        } else if s[i] == 'P' {
            dp[i + 1][2] = dp[i][0].max(dp[i][1]) + 1;
            dp[i + 1][1] = dp[i][0].max(dp[i][2]);
        } else {
            dp[i + 1][0] = dp[i][1].max(dp[i][2]) + 1;
            dp[i + 1][2] = dp[i][0].max(dp[i][1]);
        }
    }

    dp[n][0].max(dp[n][1]).max(dp[n][2])
}
