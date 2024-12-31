use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = vec![0; n];
    let mut stc = Vec::new();

    for i in (0..n - 1).rev() {
        while !stc.is_empty() && h[*stc.last().unwrap()] < h[i + 1] {
            stc.pop();
        }
        stc.push(i + 1);
        ans[i] = stc.len();
    }

    for a in ans {
        print!("{} ", a);
    }
    println!();
}
