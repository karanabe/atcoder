// https://atcoder.jp/contests/abc335/tasks/abc335_c
// D - Loong and Takahashi
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    solve(n);
}

fn solve(n: usize){
    let mut result = vec![vec![0; n]; n];
    let mut current: (usize, usize) = (0, 0);

    result[current.0][current.1] = 1;

    let mut walk = (0, 1);

    for _ in 0..n * n - 2 {
        let next = (
            current.0.wrapping_add_signed(walk.0),
            current.1.wrapping_add_signed(walk.1),
        );

        // proceed in a spiral pattern
        // Most important thought
        if next.0 >= n || next.1 >= n || result[next.0][next.1] != 0 {
            walk = (walk.1, -walk.0);
        }

        let next = (
            current.0.wrapping_add_signed(walk.0),
            current.1.wrapping_add_signed(walk.1),
        );

        result[next.0][next.1] = result[current.0][current.1] + 1;
        current = next;
    }

    for i in 0..n {
        for j in 0..n {
            if i == (n - 1) / 2 && j == (n - 1) / 2 {
                print!("T ");
            } else {
                print!("{} ", result[i][j]);
            }
        }
        println!("");
    }
}


#[cfg(test)]
mod abc335 {
    // use super::*;

    #[test]
    fn test_1() {
        assert_eq!("Yes", "Yes");
    }
}
