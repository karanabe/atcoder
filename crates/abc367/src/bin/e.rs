use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u64,
        p: [usize; n],
        a: [usize; n],
    }

    const MAX_LV: usize = 60;
    let mut dp = vec![vec![0; n]; MAX_LV];

    for i in 0..n {
        dp[0][i] = p[i] - 1;
    }

    for lv in 1..MAX_LV {
        for i in 0..n {
            dp[lv][i] = dp[lv - 1][dp[lv - 1][i]];
        }
    }

    let mut current_pos = (0..n).collect::<Vec<_>>();

    for lv in 0..MAX_LV {
        if k & 1 != 0 {
            for i in 0..n {
                current_pos[i] = dp[lv][current_pos[i]];
            }
        }
        k >>= 1;
    }

    let result = current_pos.iter().map(|&i| a[i]).collect::<Vec<_>>();
    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
