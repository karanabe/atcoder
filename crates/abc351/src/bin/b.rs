// https://atcoder.jp/contests/abc351/tasks/abc351_b
// B - Spot the Difference
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    }
    let _: String = solve(n, a, b);
}

fn solve(n: usize, a: Vec<Vec<char>>, b: Vec<Vec<char>>) -> String {
    let mut ans: String = "".to_string();

    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                ans = format!("{} {}", i + 1, j + 1);
            }
        }
    }
    println!("{ans}");
    format!("{ans}")
}
