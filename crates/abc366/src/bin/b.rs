use proconio::input;

fn gen_t(n: usize, strings: Vec<String>) -> Vec<String> {
    let max_len = strings.iter().map(|s| s.len()).max().unwrap();
    let mut result = vec![vec!['*'; n]; max_len];

    for (i, s) in strings.iter().enumerate() {
        for (j, ch) in s.chars().enumerate() {
            result[j][n - i - 1] = ch;
        }
    }

    result.into_iter()
        .map(|row| {
            let mut row_string: String = row.into_iter().collect();
            while row_string.ends_with('*') {
                row_string.pop();
            }
            row_string
        })
        .collect()
}

fn main() {
    input! {
        n: usize,
        strings: [String; n],
    }

    let result = gen_t(n, strings);
    for row in result {
        println!("{}", row);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 3;
        let strings = vec!["abc".to_string(), "de".to_string(), "fghi".to_string()];
        let expected_output = vec![
            "fda".to_string(),
            "geb".to_string(),
            "h*c".to_string(),
            "i".to_string(),
        ];
        assert_eq!(gen_t(n, strings), expected_output);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let strings = vec![
            "atcoder".to_string(),
            "beginner".to_string(),
            "contest".to_string(),
        ];
        let expected_output = vec![
            "cba".to_string(),
            "oet".to_string(),
            "ngc".to_string(),
            "tio".to_string(),
            "end".to_string(),
            "sne".to_string(),
            "ter".to_string(),
            "*r".to_string(),
        ];
        assert_eq!(gen_t(n, strings), expected_output);
    }
}
