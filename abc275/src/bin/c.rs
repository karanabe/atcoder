// https://atcoder.jp/contests/abc275/tasks/abc275_c
// C - Counting Squares
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    }
    let _: usize = solve(s);
}

fn solve(s: Vec<Vec<char>>) -> usize {
    let mut result: usize = 0;
    // let s2: Vec<Vec<char>> = s.iter().map(|x| x.chars().collect()).collect();

    for a_row in 0..9 as usize {
        for a_col in 0..9 as usize {
            for b_row in a_row..9 {
                for b_col in a_col+1..9 {
                    if s[a_row][a_col] == '#' && s[b_row][b_col] == '#' {
                        if is_square(vec![[a_row, a_col], [b_row, b_col]], &s) { result += 1; }
                    }
                }
            }
        }
    }
    println!("{}", result);
    return result;
}


fn is_square(vec: Vec<[usize; 2]>, list: &Vec<Vec<char>>) -> bool {
    let a = vec[0];
    let b = vec[1];

    let c_row = (b[0] as i32 + (a[1] as i32 - b[1] as i32).abs()) as usize;
    let c_col = (b[1] as i32 - (a[0] as i32 - b[0] as i32).abs()) as usize;
    let d_row = (c_row as i32 - (c_col as i32 - b[1] as i32).abs()) as usize;
    let d_col = (c_col as i32 - (c_row as i32 - b[0] as i32).abs()) as usize;

    if !(c_row < 9 && c_col < 9 && d_row < 9 && d_col < 9) { return false; }
    if list[c_row][c_col] == '#' && list[d_row][d_col] == '#' { return true; }
    false
}

#[cfg(test)]
mod abc275 {
    use super::*;

    #[test]
    fn test_1() {
        let s: Vec<Vec<char>> = vec![
            "##.......".to_string().chars().collect(),
            "##.......".to_string().chars().collect(),
            ".........".to_string().chars().collect(),
            ".......#.".to_string().chars().collect(),
            ".....#...".to_string().chars().collect(),
            "........#".to_string().chars().collect(),
            "......#..".to_string().chars().collect(),
            ".........".to_string().chars().collect(),
            ".........".to_string().chars().collect()
        ];
        assert_eq!(2, solve(s));
    }

    #[test]
    fn test_2() {
        let s: Vec<Vec<char>> = vec![
            ".........".to_string().chars().collect(),
            "...#.....".to_string().chars().collect(),
            ".......#.".to_string().chars().collect(),
            ".........".to_string().chars().collect(),
            ".........".to_string().chars().collect(),
            ".#.......".to_string().chars().collect(),
            ".........".to_string().chars().collect(),
            ".....#...".to_string().chars().collect(),
            ".........".to_string().chars().collect()
        ];
        assert!(is_square(vec![[1,3], [3,7]], &s));
    }

    #[test]
    fn test_3() {
        let s: Vec<Vec<char>> = vec![
            ".#.......".to_string().chars().collect(),
            "#.#......".to_string().chars().collect(),
            ".#.......".to_string().chars().collect(),
            ".........".to_string().chars().collect(),
            "....#.#.#".to_string().chars().collect(),
            ".........".to_string().chars().collect(),
            "....#.#.#".to_string().chars().collect(),
            "........#".to_string().chars().collect(),
            ".........".to_string().chars().collect()
        ];
        assert_eq!(3, solve(s));
    }
}


/*
use proconio::input;

fn main() {
    input! {
        s: [String; 9],
    }
    let _: usize = solve(&s);
}

fn solve(s: &Vec<String>) -> usize {
    let mut result: usize = 0;
    let s2: Vec<Vec<char>> = s.iter().map(|x| x.chars().collect()).collect();

    for a_row in 0..9 as usize {
        for a_col in 0..9 as usize {
            for b_row in a_row..9 {
                for b_col in a_col+1..9 {
                    if s2[a_row][a_col] == '#' && s2[b_row][b_col] == '#' {
                        if is_square(vec![[a_row, a_col], [b_row, b_col]], &s2) { result += 1; }
                    }
                }
            }
        }
    }
    println!("{}", result);
    return result;
}


fn is_square(vec: Vec<[usize; 2]>, list: &Vec<Vec<char>>) -> bool {
    let a = vec[0];
    let b = vec[1];

    let c_row = (b[0] as i32 + (a[1] as i32 - b[1] as i32).abs()) as usize;
    let c_col = (b[1] as i32 - (a[0] as i32 - b[0] as i32).abs()) as usize;
    let d_row = (c_row as i32 - (c_col as i32 - b[1] as i32).abs()) as usize;
    let d_col = (c_col as i32 - (c_row as i32 - b[0] as i32).abs()) as usize;

    if !(c_row < 9 && c_col < 9 && d_row < 9 && d_col < 9) { return false; }
    if list[c_row][c_col] == '#' && list[d_row][d_col] == '#' { return true; }
    false
}
*/
