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

    for row in dp.iter().take(MAX_LV) {
        if k & 1 != 0 {
            for value in current_pos.iter_mut() {
                *value = row[*value];
            }
        }
        k >>= 1;
    }

    let result = current_pos.iter().map(|&i| a[i]).collect::<Vec<_>>();
    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
