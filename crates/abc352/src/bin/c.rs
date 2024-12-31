// https://atcoder.jp/contests/abc352/tasks/abc352_c
// C - Standing On The Shoulders
use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let _: String = solve(n);
}

fn solve(n: usize) -> String {
    input ! { a: usize, b: usize }
    let mut max_diff = b - a;
    let mut head = b;
    let mut sholder = a;

    let mut ans: usize = 0;

    for _i in 1..n {
        input! {
            a: usize,
            b: usize,
        }
        if max_diff < (b - a) {
            ans += sholder;
            head = b;
            sholder = a;
            max_diff = b - a;
        } else {
            ans += a;
        }
    }
    ans += head;

    println!("{ans}");
    format!("{ans}")
}
