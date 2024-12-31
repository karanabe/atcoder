// Worng answer
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Isize1,Usize1,Chars,Bytes}
};
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        w: usize,
        items: [(usize, i64); n]
    }

    let mut heaps = vec![BinaryHeap::new(); w + 1];
    let mut limit = vec![0; w + 1];
    for i in 1..=w {
        limit[i] = w / i;
    }
    for &(weight, value) in items.iter() {
        let mut dec = 1;
        while value > dec
            && (heaps[weight].len() < limit[weight] || -heaps[weight].peek().unwrap() < value - dec)
        {
            heaps[weight].push(dec - value);
            if heaps[weight].len() > limit[weight] {
                heaps[weight].pop();
            }
            dec += 2;
        }
    }

    let mut items = vec![];
    for i in 1..=w {
        while let Some(v) = heaps[i].pop() {
            items.push((i, (-v) as usize));
        }
    }

    let mut dp = vec![0; w + 1];
    for &(weight, value) in items.iter() {
        for j in (weight..=w).rev() {
            dp[j] = dp[j].max(dp[j - weight] + value);
        }
    }

    println!("{}", dp[w]);
}
