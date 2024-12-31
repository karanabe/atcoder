use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut all_sum = vec![0usize; n+1];

    all_sum[1] = a[0];

    for i in 1..n {
        all_sum[i+1] = a[i] + all_sum[i];
    }

    for x in 0..q {
        let ans = all_sum[lr[x].1] - all_sum[lr[x].0 - 1];
        println!("{ans}");
    }
}
