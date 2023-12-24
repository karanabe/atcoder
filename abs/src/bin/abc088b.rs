// https://atcoder.jp/contests/abs/tasks/abc088_b
use proconio::input;
// use std::cmp::Reverse;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }
    let _: i32 = solve(&a);
}

fn solve(a: &[i32]) -> i32 {
    let mut card_deck: Vec<i32> = a.to_vec();
    card_deck.sort_by(|a, b| b.cmp(a));
    // card_deck.sort_by_key(|&x| Reverse(x));
    let mut alice: i32 = 0;
    let mut bob: i32 = 0;

    for (index, card) in card_deck.iter().enumerate() {
        if index % 2 == 0 {
            alice += card;
        } else {
            bob += card;
        }

    }
    let result = alice - bob;
    print!("{}", result);
    result
}


#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn calc_delta_test_1() {
        let a: [i32; 2] = [3, 1];
        assert_eq!(2, solve(&a));
    }

    #[test]
    fn calc_delta_test_2() {
        let a: [i32; 3] = [2, 7, 4];
        assert_eq!(5, solve(&a));
    }

    #[test]
    fn calc_delta_test_3() {
        let a: [i32; 4] = [20, 18, 2, 18];
        assert_eq!(18, solve(&a));
    }
}
