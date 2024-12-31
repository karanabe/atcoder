use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [(Usize1, Usize1, isize); q],
    }

    let mut ans = vec![0isize; n+1];

    for i in 0..q {
        ans[a[i].0] += a[i].2;
        ans[a[i].1+1] -= a[i].2;
        // println!("{:?}", ans);
    }

    for j in 1..n {
        if ans[j] > 0 {
            print!("<");
        }
        if ans[j] == 0 {
            print!("=");
        }
        if ans[j] < 0 {
            print!(">");
        }
    }
    println!("");
}
