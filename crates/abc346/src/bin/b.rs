use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }
    let s = "wbwbwwbwbwbw".repeat(400).chars().collect_vec();
    for i in 0..=s.len() - w - b {
        if s[i..i + w + b].iter().filter(|&&c| c == 'w').count() == w {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
