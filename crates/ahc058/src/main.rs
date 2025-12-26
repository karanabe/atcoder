#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::line::LineSource,
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{Ordering, max, min};

#[allow(unused_imports)]
use ac_library::{
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
    Dsu,
    // new(size: usize) -> Self
    // merge(&mut self, a: usize, b: usize) -> usize
    // same(&mut self, a: usize, b: usize) -> bool
    // leader(&mut self, a: usize) -> usize
    // size(&mut self, a: usize) -> usize
    // groups(&mut self) -> Vec<Vec<usize>>
    FenwickTree,
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    Max,
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&mut self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree,
    math,
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

#[allow(unused_imports)]
use std::io::{self, BufReader, StdinLock, Write};

#[allow(unused_imports)]
use rand::{Rng, SeedableRng, prelude::*, rngs::StdRng, seq::SliceRandom, thread_rng};
#[allow(unused_imports)]
use rand_distr::{Distribution, Normal};
#[allow(unused_imports)]
use std::time::{Duration, Instant};

const N: usize = 10;
const L: usize = 4;
const ACTIONS: usize = N * L;

#[derive(Clone)]
struct XorShift64 {
    x: u64,
}
impl XorShift64 {
    #[inline(always)]
    fn new(seed: u64) -> Self {
        Self { x: seed }
    }
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let mut x = self.x;
        x ^= x << 7;
        x ^= x >> 9;
        self.x = x;
        x
    }
    #[inline(always)]
    fn next_f64(&mut self) -> f64 {
        let v = self.next_u64() >> 11;
        (v as f64) * (1.0 / ((1u64 << 53) as f64))
    }
    #[inline(always)]
    fn gen_usize(&mut self, n: usize) -> usize {
        (self.next_u64() as usize) % n
    }
}

#[derive(Clone, Copy)]
struct Params {
    alpha0: f64,
    pot_div: u128,
    cost_div: u128,
    top_k: usize,
    best_prob: f64,
    shake_prob: f64,
}

#[inline(always)]
fn combs(r: usize) -> (u128, u128, u128, u128) {
    let r = r as u128;
    let c1 = r;
    let c2 = if r >= 2 { r * (r - 1) / 2 } else { 0 };
    let c3 = if r >= 3 { r * (r - 1) * (r - 2) / 6 } else { 0 };
    let c4 = if r >= 4 {
        r * (r - 1) * (r - 2) * (r - 3) / 24
    } else {
        0
    };
    (c1, c2, c3, c4)
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn gain_stop(
    a: u128,
    b0: u128,
    b1: u128,
    b2: u128,
    b3: u128,
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    c1: u128,
    c2: u128,
    c3: u128,
    c4: u128,
) -> u128 {
    if p0 == 0 {
        return 0;
    }
    let ap0 = a * (p0 as u128);
    let f0 = ap0;
    let f1 = f0 * (p1 as u128);
    let f2 = f1 * (p2 as u128);
    let f3 = f2 * (p3 as u128);

    b0 * (f0 * c1) + b1 * (f1 * c2) + b2 * (f2 * c3) + b3 * (f3 * c4)
}

#[inline(always)]
fn apply_action(
    act: i8,
    apples: &mut u128,
    b: &mut [[u128; N]; L],
    p: &mut [[u32; N]; L],
    a: &[u128; N],
    c: &[[u128; N]; L],
) -> i8 {
    let mut act = act;
    if act >= 0 {
        let idx = act as usize;
        let i = idx / N;
        let j = idx % N;
        let cost = c[i][j] * (p[i][j] as u128 + 1);
        if cost <= *apples {
            *apples -= cost;
            p[i][j] += 1;
        } else {
            act = -1;
        }
    } else {
        act = -1;
    }

    for j in 0..N {
        *apples += a[j] * b[0][j] * (p[0][j] as u128);
    }
    for j in 0..N {
        b[0][j] += b[1][j] * (p[1][j] as u128);
    }
    for j in 0..N {
        b[1][j] += b[2][j] * (p[2][j] as u128);
    }
    for j in 0..N {
        b[2][j] += b[3][j] * (p[3][j] as u128);
    }

    act
}

#[allow(clippy::too_many_arguments)]
fn build_from_prefix(
    base: Option<&[i8]>,
    cut: usize,
    params: Params,
    rng: &mut XorShift64,
    a: &[u128; N],
    c: &[[u128; N]; L],
    t: usize,
    k: u128,
) -> (Vec<i8>, u128) {
    let mut actions = vec![-1i8; t];

    if let Some(b) = base {
        actions[..cut].copy_from_slice(&b[..cut]);
        if cut > 0 && rng.next_f64() < params.shake_prob {
            let m = 1 + rng.gen_usize(3);
            let w = 30.min(cut);
            for _ in 0..m {
                let pos = cut - w + rng.gen_usize(w);
                if rng.next_f64() < 0.12 {
                    actions[pos] = -1;
                } else {
                    actions[pos] = rng.gen_usize(ACTIONS) as i8;
                }
            }
        }
    }

    let mut apples: u128 = k;
    let mut b = [[1u128; N]; L];
    let mut p = [[0u32; N]; L];

    for action in actions.iter_mut().take(cut) {
        let act = *action;
        *action = apply_action(act, &mut apples, &mut b, &mut p, a, c);
    }

    for (turn, action) in actions.iter_mut().enumerate().take(t).skip(cut) {
        let r = t - turn;
        let (c1, c2, c3, c4) = combs(r);

        let mut gain_before = [0u128; N];
        let mut base_total: u128 = 0;
        for j in 0..N {
            let g = gain_stop(
                a[j], b[0][j], b[1][j], b[2][j], b[3][j], p[0][j], p[1][j], p[2][j], p[3][j], c1,
                c2, c3, c4,
            );
            gain_before[j] = g;
            base_total += g;
        }

        let progress = (turn as f64) / (t as f64);
        let alpha_turn = params.alpha0 * (1.0 - progress);
        let use_potential = rng.next_f64() < alpha_turn;

        let mut cands: Vec<(u128, i8)> = Vec::with_capacity(ACTIONS + 1);

        let mut metric_noop = apples + base_total;

        let mut has_feasible = false;

        for i in 0..L {
            for j in 0..N {
                let cost = c[i][j] * (p[i][j] as u128 + 1);
                if cost > apples {
                    continue;
                }
                has_feasible = true;

                let mut p0 = p[0][j];
                let mut p1 = p[1][j];
                let mut p2 = p[2][j];
                let mut p3 = p[3][j];
                match i {
                    0 => p0 += 1,
                    1 => p1 += 1,
                    2 => p2 += 1,
                    3 => p3 += 1,
                    _ => unreachable!(),
                }

                let g_after = gain_stop(
                    a[j], b[0][j], b[1][j], b[2][j], b[3][j], p0, p1, p2, p3, c1, c2, c3, c4,
                );

                let final_act = (apples - cost) + (base_total - gain_before[j] + g_after);

                let mut metric = final_act;

                if use_potential {
                    let depth = i;

                    let g_before = gain_before[j];
                    let actual_delta = g_after - g_before;

                    let mut extra: u128 = 0;
                    let mut act_cost: u128 = 0;

                    if depth >= 1 && p[0][j] == 0 {
                        act_cost += c[0][j];
                    }
                    if depth >= 2 && p[1][j] == 0 {
                        act_cost += c[1][j];
                    }
                    if depth >= 3 && p[2][j] == 0 {
                        act_cost += c[2][j];
                    }

                    if depth > 0 {
                        let fp0 = if depth >= 1 { p[0][j].max(1) } else { p[0][j] };
                        let fp1 = if depth >= 2 { p[1][j].max(1) } else { p[1][j] };
                        let fp2 = if depth >= 3 { p[2][j].max(1) } else { p[2][j] };
                        let fp3 = p[3][j];

                        let g_floor_before = gain_stop(
                            a[j], b[0][j], b[1][j], b[2][j], b[3][j], fp0, fp1, fp2, fp3, c1, c2,
                            c3, c4,
                        );

                        let mut q0 = fp0;
                        let mut q1 = fp1;
                        let mut q2 = fp2;
                        let mut q3 = fp3;
                        match i {
                            0 => q0 = p0,
                            1 => q1 = p1,
                            2 => q2 = p2,
                            3 => q3 = p3,
                            _ => unreachable!(),
                        }

                        let g_floor_after = gain_stop(
                            a[j], b[0][j], b[1][j], b[2][j], b[3][j], q0, q1, q2, q3, c1, c2, c3,
                            c4,
                        );

                        let floor_delta = g_floor_after - g_floor_before;
                        extra = floor_delta.saturating_sub(actual_delta);
                    }

                    metric = metric
                        .saturating_add(extra / params.pot_div)
                        .saturating_sub(act_cost / params.cost_div);
                }

                cands.push((metric, (i * N + j) as i8));
            }
        }

        if has_feasible {
            let idle_div = if turn < t / 2 { 200u128 } else { 1200u128 };
            metric_noop = metric_noop.saturating_sub(metric_noop / idle_div);
        }
        cands.push((metric_noop, -1));

        cands.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let k_take = params.top_k.min(cands.len());

        let chosen = if rng.next_f64() < params.best_prob {
            cands[0].1
        } else {
            cands[rng.gen_usize(k_take)].1
        };

        *action = chosen;
        *action = apply_action(chosen, &mut apples, &mut b, &mut p, a, c);
    }

    (actions, apples)
}

fn insert_elite(elite: &mut Vec<(u128, Vec<i8>)>, cand: (u128, Vec<i8>), keep: usize) {
    elite.push(cand);
    elite.sort_unstable_by(|x, y| y.0.cmp(&x.0));
    elite.dedup_by(|a, b| a.0 == b.0);
    if elite.len() > keep {
        elite.truncate(keep);
    }
}

fn sample_params(rng: &mut XorShift64, progress: f64, is_rebuild: bool) -> Params {
    let mut alpha0 = if is_rebuild { 0.70 } else { 0.88 };
    alpha0 -= 0.55 * progress;
    alpha0 += (rng.next_f64() - 0.5) * 0.10;
    alpha0 = alpha0.clamp(0.0, 0.98);

    let pot_div_choices = [1u128, 2, 4, 8];
    let cost_div_choices = [4u128, 8, 16, 32];

    let super_aggressive = rng.next_f64() < (0.12 * (1.0 - progress));

    let pot_div = if super_aggressive {
        1
    } else {
        pot_div_choices[rng.gen_usize(pot_div_choices.len())]
    };
    let cost_div = if super_aggressive {
        32
    } else {
        cost_div_choices[rng.gen_usize(cost_div_choices.len())]
    };

    let top_k = if is_rebuild {
        6 + rng.gen_usize(5)
    } else {
        7 + rng.gen_usize(6)
    };

    let mut best_prob = if is_rebuild { 0.70 } else { 0.58 };
    best_prob += (rng.next_f64() - 0.5) * 0.18;
    best_prob = best_prob.clamp(0.45, 0.90);

    let shake_prob = if is_rebuild { 0.22 } else { 0.06 };

    Params {
        alpha0,
        pot_div,
        cost_div,
        top_k,
        best_prob,
        shake_prob,
    }
}

//#[fastout]
fn main() {
    input! {
        n: usize, l: usize, t: usize, k_in: u64,
        a_in: [u64; n],
        c_in: [[u64; n]; l],
    }
    assert!(n == N && l == L);

    let mut a = [0u128; N];
    for j in 0..N {
        a[j] = a_in[j] as u128;
    }
    let mut c = [[0u128; N]; L];
    for i in 0..L {
        for j in 0..N {
            c[i][j] = c_in[i][j] as u128;
        }
    }
    let k = k_in as u128;

    let start = Instant::now();
    let time_limit = 1.90_f64;

    let seed = (a[0] as u64)
        ^ ((c[0][0] as u64).wrapping_mul(0x9E3779B97F4A7C15))
        ^ (start.elapsed().as_nanos() as u64);
    let mut rng = XorShift64::new(seed ^ 0xD1B54A32D192ED03);

    let base_params = Params {
        alpha0: 0.0,
        pot_div: 1,
        cost_div: 1,
        top_k: 1,
        best_prob: 1.0,
        shake_prob: 0.0,
    };
    let (base_actions, base_apples) =
        build_from_prefix(None, 0, base_params, &mut rng, &a, &c, t, k);

    let elite_keep = 6usize;
    let mut elite: Vec<(u128, Vec<i8>)> = Vec::new();
    insert_elite(&mut elite, (base_apples, base_actions), elite_keep);

    while start.elapsed().as_secs_f64() < time_limit {
        let elapsed = start.elapsed().as_secs_f64();
        let progress = (elapsed / time_limit).min(1.0);

        let do_scratch =
            elite.len() < elite_keep || rng.next_f64() < (0.38 * (1.0 - progress) + 0.12);

        let (cand_actions, cand_apples) = if do_scratch {
            let params = sample_params(&mut rng, progress, false);
            build_from_prefix(None, 0, params, &mut rng, &a, &c, t, k)
        } else {
            let m = elite.len();
            let u = rng.next_f64();
            let mut idx = ((u * u) * (m as f64)) as usize;
            if idx >= m {
                idx = m - 1;
            }
            let base = &elite[idx].1;

            let u2 = rng.next_f64();
            let mut cut = ((u2 * u2) * (t as f64)) as usize;
            if cut >= t {
                cut = t - 1;
            }
            if cut < 5 {
                cut = 5.min(t - 1);
            }

            let params = sample_params(&mut rng, progress, true);
            build_from_prefix(Some(base), cut, params, &mut rng, &a, &c, t, k)
        };

        if cand_apples > elite.last().map(|x| x.0).unwrap_or(0) || elite.len() < elite_keep {
            insert_elite(&mut elite, (cand_apples, cand_actions), elite_keep);
        }
    }

    let best_actions = &elite[0].1;
    for &act in best_actions.iter() {
        if act < 0 {
            println!("-1");
        } else {
            let idx = act as usize;
            let i = idx / N;
            let j = idx % N;
            println!("{} {}", i, j);
        }
    }
}
