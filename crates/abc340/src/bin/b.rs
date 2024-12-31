// https://atcoder.jp/contests/abc340/tasks/abc340_b
// B - Append
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let _: String = solve(n);
}

fn solve(n: usize) -> String {
    let mut result: Vec<usize> = vec![];
    for _i in 0..n {
        input! { q: usize, xk: usize}
        if q == 1 {
            result.push(xk);
        }
        if q == 2{
            println!("{}", result.get(result.len() - xk).unwrap());
        }

    }
    format!("test")
}

