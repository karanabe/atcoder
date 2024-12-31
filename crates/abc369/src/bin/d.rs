use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    if n == 1 {
        println!("{}", a[0]);
        return
    }

    let mut dp = vec![(0, 0); n + 1];

    dp[1] = (0, a[0]);
    dp[2] = (dp[1].1 + a[1]*2, dp[1].0 + a[1]);

    if n == 2 {
        println!("{}", dp[2].0.max(dp[2].1));
        return
    }

    for i in 2..n {
        let dp_index = i+1;

        dp[dp_index].0 = (a[i]*2 + dp[i].1).max(a[i]*2 + dp[i-1].1);
        dp[dp_index].1 = (a[i] + dp[i].0).max(a[i] + dp[i-1].0);
    }

    println!("{}", dp[n].0.max(dp[n].1));
}
