use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut last_occr = vec![0usize; n + 1];
    let mut ans: u64 = 0;

    for (i, &v) in a.iter().enumerate() {
        let last_v = last_occr[v];
        let c = (i + 1 - last_v) as u64 * (n - i) as u64;
        ans += c;
        last_occr[v] = i + 1;
    }

    println!("{}", ans);
}
