// Worng answer
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Isize1,Usize1,Chars,Bytes}
};
use std::cmp::Reverse;
use itertools::Itertools;


fn main() {
    input! {n: usize, m: usize, k: i64, a: [i64; n]}

    let sum = a.iter().sum::<i64>();
    let mut a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<Vec<_>>();
    a.sort_unstable_by_key(|v| Reverse(v.0));

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i].0;
    }

    let mut res = vec![-1; n];
    for (j, &(na, i)) in a.iter().enumerate() {
        let (mut l, mut r) = (-1, k - sum + 1);
        while r - l > 1 {
            let mid = (r + l) / 2;
            let rem = k - sum - mid;
            let na = na + mid;
            let pos = a.partition_point(|v| v.0 > na);
            if pos >= m {
                l = mid;
                continue;
            }

            if pos == n - 1 {
                r = mid;
                continue;
            }

            let lack = m - pos;
            let s = if (pos..pos + lack).contains(&j) {
                if pos + lack + 1 > n {
                    r = mid;
                    continue;
                }
                cum[pos + lack + 1] - cum[pos] - (na - mid)
            } else {
                cum[pos + lack] - cum[pos]
            };
            if s + rem >= (na + 1) * lack as i64 {
                l = mid;
            } else {
                r = mid;
            }
        }

        if r != k - sum + 1 {
            res[i] = r;
        }
    }

    println!("{}", res.into_iter().map(|r| r as i64).join(" "))
}
