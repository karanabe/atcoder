// https://atcoder.jp/contests/abc339/tasks/abc339_b
// B - Langton's Takahashi
use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
        n: usize,
    }
    let _ = solve(h, w, n);
}

fn solve(h: i32, w: i32, n: usize) {
    let mut a = vec![vec!["."; w as usize]; h as usize];
    let mut current: (i32, i32) = (0, 0);
    let mut next: (i32, i32) = (-1, 0);

    for _ in 0..n {
        if a[current.0 as usize][current.1 as usize] == "." {
            a[current.0 as usize][current.1 as usize] = "#";
            next = (next.1, -next.0);
        } else {
            a[current.0 as usize][current.1 as usize] = ".";
            next = (-next.1, next.0);
        }

        current = (
            current.0 + next.0,
            current.1 + next.1
        );

        if current.0 == h {
            current.0 = 0;
        } else if current.0 < 0 {
            current.0 = h-1;
        } else if current.1 == w {
            current.1 = 0;
        } else if current.1 < 0 {
            current.1 = w-1;
        }
    }

    for row in &a {
        let s = row.join("");
        println!("{s}");
    }
}

#[cfg(test)]
mod abc339 {


}
