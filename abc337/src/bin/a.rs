// https://atcoder.jp/contests/abc335/tasks/abc337_a
// A - Scoreboard
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [[usize; 2]; n],
    }
    let _: String = solve(n, xy);
}

fn solve(_n: usize, xy: Vec<Vec<usize>>) -> String {
    let mut takahashi = 0;
    let mut aoki = 0;
    for arr in xy.iter() {
        takahashi += arr[0];
        aoki += arr[1];
    }
    if takahashi > aoki {
        println!("Takahashi");
        return "Takahashi".to_string();
    } else if aoki > takahashi {
        println!("Aoki");
        return "Aoki".to_string();
    } else {
        println!("Draw");
    }
    "Draw".to_string()
}

#[cfg(test)]
mod abc337 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 4;
        let xy: Vec<Vec<usize>> = vec![vec![10, 2], vec![10, 1], vec![10, 2], vec![3, 2]];
        assert_eq!("Takahashi", solve(n, xy));
    }

    #[test]
    fn test_2() {
        let n: usize = 2;
        let xy: Vec<Vec<usize>> = vec![vec![1, 1], vec![2, 2]];
        assert_eq!("Draw", solve(n, xy));
    }
}
