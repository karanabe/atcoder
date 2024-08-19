// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_af
// 034 - Nearest Points
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(f64, f64); n]
    }

    let result = solve(n, a);
    println!("{:.12}", result);
}

fn solve(n: usize, a: Vec<(f64, f64)>) -> f64 {
    let mut ans = (1 << 30) as f64;

    for i in 0..n {
        for j in i + 1..n {
            let x = a[j].0 - a[i].0;
            let y = a[j].1 - a[i].1;
            ans = ans.min((x * x + y * y).sqrt());
        }
    }
    ans
}
