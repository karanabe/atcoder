#[allow(unused_imports)]
use itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        s: [String; 12]
    }

    let mut ans = 0;

    for (index, word) in s.iter().enumerate() {
        if word.len() == index + 1 {
            ans += 1;
        }
    }

    println!("{ans}");
}
