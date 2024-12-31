// https://atcoder.jp/contests/abc335/tasks/abc335_b
// B - Tetrahedral Number
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    solve(n);
}

fn solve(n: usize) {
    for x in 0..=n {
        for y in 0..=n {
            for z in 0..=n {
                if x+y+z <= n {
                    println!("{} {} {}", x, y, z);
                }
            }
        }
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
