use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        key_hand: [(i32, Chars); n],
    }

    let mut left_hand_pos = -1;
    let mut right_hand_pos = -1;
    let mut fatigue = 0;

    for (key, hand) in key_hand {
        let hand = hand[0];
        if hand == 'L' {
            if left_hand_pos == -1 {
                left_hand_pos = key;
            } else {
                fatigue += (key - left_hand_pos).abs();
                left_hand_pos = key;
            }
        } else if hand == 'R' {
            if right_hand_pos == -1 {
                right_hand_pos = key;
            } else {
                fatigue += (key - right_hand_pos).abs();
                right_hand_pos = key;
            }
        }
    }

    println!("{}", fatigue);
}
