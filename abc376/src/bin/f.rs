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


fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, usize); q],
    }

    let mut ans = 0;
    let mut l = 1;
    let mut r = 2;

    for (h, t) in ht {
        let (h_pos, other_pos) = if h == 'L' { (l, r) } else { (r, l) };

        if h_pos == t {
            // NOP
        } else {
            let steps_cw = simulate(h_pos, other_pos, t, n, true);
            let steps_ccw = simulate(h_pos, other_pos, t, n, false);

            let min_steps = steps_cw.min(steps_ccw);
            ans += min_steps;

            if h == 'L' {
                l = t;
                r = if steps_cw <= steps_ccw {
                    steps_cw_other_pos(h_pos, other_pos, t, n, true)
                } else {
                    steps_cw_other_pos(h_pos, other_pos, t, n, false)
                };
            } else {
                r = t;
                l = if steps_cw <= steps_ccw {
                    steps_cw_other_pos(h_pos, other_pos, t, n, true)
                } else {
                    steps_cw_other_pos(h_pos, other_pos, t, n, false)
                };
            }
        }
    }

    println!("{}", ans);
}


fn simulate(mut h_pos: usize, mut other_pos: usize, t: usize, n: usize, clockwise: bool) -> usize {
    let mut steps = 0;

    while h_pos != t {
        let next_h_pos = if clockwise {
            if h_pos == n { 1 } else { h_pos + 1 }
        } else {
            if h_pos == 1 { n } else { h_pos - 1 }
        };

        if next_h_pos == other_pos {
            let next_other_pos = if clockwise {
                if other_pos == n { 1 } else { other_pos + 1 }
            } else {
                if other_pos == 1 { n } else { other_pos - 1 }
            };

            steps += 1;
            other_pos = next_other_pos;
        }

        steps += 1;
        h_pos = next_h_pos;
    }

    steps
}

fn steps_cw_other_pos(mut h_pos: usize, mut other_pos: usize, t: usize, n: usize, clockwise: bool) -> usize {
    while h_pos != t {
        let next_h_pos = if clockwise {
            if h_pos == n { 1 } else { h_pos + 1 }
        } else {
            if h_pos == 1 { n } else { h_pos - 1 }
        };

        if next_h_pos == other_pos {
            let next_other_pos = if clockwise {
                if other_pos == n { 1 } else { other_pos + 1 }
            } else {
                if other_pos == 1 { n } else { other_pos - 1 }
            };

            other_pos = next_other_pos;
        }

        h_pos = next_h_pos;
    }

    other_pos
}
