#[allow(unused_imports)]
use proconio::{
    input,
    input_interactive,
    fastout,
    source::line::LineSource,
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
    max,
    Ordering
};

#[allow(unused_imports)]
use ac_library::{
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
    math,
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&mut self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree,
    Max
};

#[allow(unused_imports)]
use num::{
    BigInt,
    Zero
};

#[allow(unused_imports)]
use std::io::{stdout, Write};

#[allow(unused_imports)]
use rand::{
    rngs::StdRng,
    Rng,
    thread_rng,
    SeedableRng
};
#[allow(unused_imports)]
use rand_distr::{Normal, Distribution};
#[allow(unused_imports)]
use std::time::{Instant, Duration};


static SIM_STEPS: usize = 10_000;

fn simulate(n: usize, sol: &[(usize, usize)], steps: usize) -> (Vec<usize>, usize) {
    let mut count = vec![0; n];

    let mut current = 0;
    count[current] += 1;

    for _ in 1..steps {
        let t = count[current];
        if t % 2 == 1 {
            current = sol[current].0;
        } else {
            current = sol[current].1;
        }
        count[current] += 1;
    }
    (count, current)
}

fn compute_error(n: usize, count: &[usize], t_target: &[usize], l: usize, sim_steps: usize) -> usize {
    let scale = (l as f64) / (sim_steps as f64);
    let mut err_sum = 0usize;
    for i in 0..n {
        let approx = (count[i] as f64) * scale;
        let diff = approx - (t_target[i] as f64);
        err_sum += diff.abs() as usize;
    }
    err_sum
}

fn main() {
    input! {
        n: usize,
        l: usize,
        t_vals: [usize; n],
    }

    let mut rng = thread_rng();
    let mut sol = vec![(0, 0); n];
    for i in 0..n {
        let a_i = rng.gen_range(0..n);
        let b_i = rng.gen_range(0..n);
        sol[i] = (a_i, b_i);
    }

    let (count_init, _) = simulate(n, &sol, SIM_STEPS);
    let mut best_error = compute_error(n, &count_init, &t_vals, l, SIM_STEPS);
    let mut best_sol = sol.clone();

    let start_time = Instant::now();
    let time_limit = Duration::from_millis(1900);
    let mut iteration_count = 0;

    let temp_max: f64 = 1000.0;
    let temp_min: f64 = 1.0;
    let max_iter = 50000;


    while iteration_count < max_iter {
        iteration_count += 1;

        if start_time.elapsed() >= time_limit {
            break;
        }

        let progress = iteration_count as f64 / max_iter as f64;
        let temperature = temp_max.powf(1.0 - progress) * temp_min.powf(progress);

        let i = rng.gen_range(0..n);
        let change_ab = rng.gen_bool(0.5);
        let old_val = if change_ab { sol[i].0 } else { sol[i].1 };

        let new_val = rng.gen_range(0..n);

        if change_ab {
            sol[i].0 = new_val;
        } else {
            sol[i].1 = new_val;
        }

        let (count_test, _) = simulate(n, &sol, SIM_STEPS);
        let new_error = compute_error(n, &count_test, &t_vals, l, SIM_STEPS);


        let diff = new_error as i64 - best_error as i64;
        if diff < 0 {
            if new_error < best_error {
                best_error = new_error;
                best_sol = sol.clone();
            }
        } else {

            let prob = f64::exp(-(diff as f64) / temperature);
            if rng.gen_bool(prob) {

            } else {
                if change_ab {
                    sol[i].0 = old_val;
                } else {
                    sol[i].1 = old_val;
                }
            }
        }
    }

    for i in 0..n {
        println!("{} {}", best_sol[i].0, best_sol[i].1);
    }
}
