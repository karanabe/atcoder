// https://atcoder.jp/contests/adt_easy_20240123_3/tasks/abc284_a
// B - Sequence of Strings
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }
    let _: String = solve(n, s);
}

fn solve(_n: usize, mut s: Vec<String>) -> String {
    s.reverse();
    for out in s.iter() {
        println!("{out}");
    }
    format!("test")
}


#[cfg(test)]
mod adt_20240123_03 {
    #[test]
    fn test_1() {
        assert_eq!("test", "test");
    }
}
