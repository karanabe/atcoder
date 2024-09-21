use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans: String = s.chars().filter(|&c| c != '.').collect();

    println!("{}", ans);
}
