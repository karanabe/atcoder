// https://atcoder.jp/contests/abc351/tasks/abc351_a
// A - The bottom of the ninth
use proconio::input;

fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8],
    }
    let _: String = solve(a, b);
}

fn solve(a: Vec<usize>, b: Vec<usize>) -> String {
    let ans: usize = a.iter().sum::<usize>() - b.iter().sum::<usize>() + 1;

    println!("{ans}");
    format!("{ans}")
}

#[cfg(test)]
mod abc352 {
    use super::*;

    #[test]
    fn test_1() {
        let a = vec![0, 1, 0, 1, 2, 2, 0, 0, 1];
        let b = vec![1, 1, 0, 0, 0, 0, 1, 0];
        assert_eq!("5", solve(a, b));
    }

    #[test]
    fn test_2() {
        let a = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let b = vec![0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!("1", solve(a, b));
    }
}
