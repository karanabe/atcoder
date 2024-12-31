#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Isize1,Usize1,Chars,Bytes}
};
#[allow(unused_imports)]
use itertools;

fn main() {
    input! {
        s: String,
    }

    let mut pos = [0; 26];
    for (i, c) in s.chars().enumerate() {
        let idx = (c as u8 - b'A') as usize;
        pos[idx] = i as i32 + 1;
    }

    let mut ttl_dist = 0;
    let mut curr_pos = pos[('A' as u8 - b'A') as usize];

    for c in ('B' as u8)..=('Z' as u8) {
        let next_position = pos[(c - b'A') as usize];
        ttl_dist += (next_position - curr_pos).abs();
        curr_pos = next_position;
    }

    println!("{}", ttl_dist);
}
