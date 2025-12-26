#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

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
    let mut curr_pos = pos[0];

    for c in b'B'..=b'Z' {
        let next_position = pos[(c - b'A') as usize];
        ttl_dist += (next_position - curr_pos).abs();
        curr_pos = next_position;
    }

    println!("{}", ttl_dist);
}
