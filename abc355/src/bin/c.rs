// https://atcoder.jp/contests/abc355/tasks/abc355_c
// C - Bingo 2
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t]
    }
    let _: String = solve(n, t, a);
}

fn solve(n: usize, t: usize, a: Vec<usize>) -> String {
    let turns = a.clone();
    let mut rows = vec![0; n];
    let mut cols = vec![0; n];
    let mut diag = 0;
    let mut anti_diag = 0;

    for (i, &number) in turns.iter().enumerate() {
        let row = (number - 1) / n;
        let col = (number - 1) % n;

        rows[row] += 1;
        cols[col] += 1;

        if row == col {
            diag += 1;
        }

        if row + col == n - 1 {
            anti_diag += 1;
        }


        if rows[row] == n || cols[col] == n || diag == n || anti_diag == n {
            println!("{}", i + 1);
            return format!("{}", i + 1);
        }
    }

    println!("-1");
    format!("-1")

}


#[cfg(test)]
mod abc355 {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let t = 5;
        let a = vec![5, 1, 8, 9, 7];
        assert_eq!("4", solve(n, t, a));
    }

    #[test]
    fn test_2() {
        let n = 3;
        let t = 5;
        let a = vec![4, 2, 9, 7, 5];
        assert_eq!("-1", solve(n, t, a));
    }

    #[test]
    fn test_3() {
        let n = 4;
        let t = 12;
        let a = vec![13, 9, 6, 5, 2, 7, 16, 14, 8, 3, 10, 11];
        assert_eq!("9", solve(n, t, a));
    }
}
