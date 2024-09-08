use proconio::input;
use std::collections::HashMap;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    }

    let mut dp: HashMap<isize, usize> = HashMap::new();
    dp.insert(0, 1);
    let mut sum = 0;
    let mut tot = 1;

    for (i, &ai) in a.iter().enumerate() {
        sum += ai;
        let banned = sum - k;
        let mut curr = tot;
        if dp.contains_key(&banned) {
           curr = (curr + MOD - dp[&banned]) % MOD;
        }

        *dp.entry(sum).or_insert(0) += curr;
        *dp.entry(sum).or_insert(0) %= MOD;
        tot = (tot + curr) % MOD;

        println!("{sum}");

        if i == n - 1 {
            println!("{}", curr);
            return;
        }
    }
}
