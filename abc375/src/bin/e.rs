#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Isize1,Usize1,Chars,Bytes}
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{
    VecDeque,
    LinkedList,
    HashMap,
    BTreeMap,
    HashSet,
    BTreeSet,
    BinaryHeap
};

#[allow(unused_imports)]
use std::cmp::{
    min,
    max
};

fn main() {
    input! {
        n: usize,
        people: [(usize, usize); n],
    }

    let total_strength: usize = people.iter().map(|&(_, b)| b).sum();
    let target_strength = total_strength / 3;

    if total_strength % 3 != 0 {
        println!("-1");
        return;
    }

    let mut team_strength = vec![0; 4];
    for &(team, strength) in &people {
        team_strength[team] += strength;
    }

    let max_strength = total_strength;
    let mut dp = vec![vec![None; max_strength + 1]; max_strength + 1];
    dp[team_strength[1]][team_strength[2]] = Some(0);

    for (_i, &(team_i, strength_i)) in people.iter().enumerate() {
        let mut new_dp = dp.clone();
        for s1 in 0..=max_strength {
            for s2 in 0..=max_strength {
                if let Some(count) = dp[s1][s2] {
                    for new_team in 1..=3 {
                        if new_team == team_i {
                            continue;
                        }

                        let mut new_s1 = s1;
                        let mut new_s2 = s2;

                        if team_i == 1 {
                            new_s1 -= strength_i;
                        } else if team_i == 2 {
                            new_s2 -= strength_i;
                        }

                        if new_team == 1 {
                            new_s1 += strength_i;
                        } else if new_team == 2 {
                            new_s2 += strength_i;
                        }

                        if new_s1 > max_strength || new_s2 > max_strength {
                            continue;
                        }

                        let new_count = count + 1;

                        if new_dp[new_s1][new_s2].is_none()
                            || new_dp[new_s1][new_s2].unwrap() > new_count
                        {
                            new_dp[new_s1][new_s2] = Some(new_count);
                        }
                    }
                }
            }
        }
        dp = new_dp;
    }

    if let Some(min_changes) = dp[target_strength][target_strength] {
        println!("{}", min_changes);
    } else {
        println!("-1");
    }
}
