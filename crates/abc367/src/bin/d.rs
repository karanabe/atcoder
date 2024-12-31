use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut b = a.repeat(2);
    b.insert(0, 0);
    for i in 0..b.len() - 1 {
        b[i + 1] += b[i];
        b[i + 1] %= m;
    }

    let mut cnt = vec![0; m + 1];
    for i in 0..n {
        cnt[b[i]] += 1;
    }

    let mut res = 0usize;
    for i in 0..n {
        cnt[b[i]] -= 1;
        res += cnt[b[i]];
        cnt[b[i + n]] += 1;
    }

    println!("{res}")
}
