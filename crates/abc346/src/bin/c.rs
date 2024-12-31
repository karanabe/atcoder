use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();
    a.dedup();
    let a = a.into_iter().filter(|&a| a <= k).collect::<Vec<_>>();

    println!("{}", k * (k + 1) / 2 - a.iter().sum::<usize>())
}
