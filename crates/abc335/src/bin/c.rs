// https://atcoder.jp/contests/abc335/tasks/abc335_c
// C - Loong Tracking
// https://atcoder.jp/contests/abc335/tasks/abc335_b
// C - Loong Tracking
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    solve(n, q);
}

fn solve(n: usize, q: usize) {
    let mut a = VecDeque::from_iter((1..=n).map(|x| (x as i32, 0)));

    for _j in 0..q {
        input! { t: usize }
        if t == 1 {
            input! { c: char }
            let mut top = a[0];
            match c {
                'R' => top.0 += 1,
                'L' => top.0 -= 1,
                'U' => top.1 += 1,
                'D' => top.1 -= 1,
                _ => todo!(),
            }
            a.push_front(top);
            a.pop_back();
        }
        if t == 2 {
            input! { p: usize }
            println!("{} {}", a[p-1].0, a[p-1].1);
        }
    }
}
