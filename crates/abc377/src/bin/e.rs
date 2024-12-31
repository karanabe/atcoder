#[allow(unused_imports)]
use proconio::{
    input,
    fastout,
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

fn mod_pow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut ans = 1 % modulo;
    base %= modulo;
    while exp > 0 {
        if exp % 2 == 1 {
            ans = ans * base % modulo;
        }
        base = base * base % modulo;
        exp /= 1 << 1;
    }
    ans
}

fn main() {
    input! {
        n: usize,
        k: u64,
        p: [usize; n],
    }

    let mut visited = vec![false; n];
    let mut pos_cyc = vec![0; n];
    let mut cyc_ele = vec![Vec::new(); n];
    let mut ele_to_cyc = vec![0; n];

    let mut cycle_count = 0;

    for i in 0..n {
        if !visited[i] {
            let mut cycle = Vec::new();
            let mut x = i;
            while !visited[x] {
                visited[x] = true;
                pos_cyc[x] = cycle.len();
                ele_to_cyc[x] = cycle_count;
                cycle.push(p[x]);
                x = p[x] - 1;
            }
            cyc_ele[cycle_count] = cycle;
            cycle_count += 1;
        }
    }

    let mut ans = vec![0; n];

    for i in 0..n {
        let cycle_idx = ele_to_cyc[i];
        let cycle = &cyc_ele[cycle_idx];
        let l = cycle.len() as u64;

        let s = (mod_pow(2, k, l) + l - 1) % l;

        let idx_in_cycle = pos_cyc[i] as u64;
        let new_idx = ((idx_in_cycle + s) % l) as usize;
        ans[i] = cycle[new_idx];
    }

    for i in 0..n {
        print!("{}{}", ans[i], if i == n - 1 { "\n" } else { " " });
    }
}
