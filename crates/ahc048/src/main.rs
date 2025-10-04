#[allow(unused_imports)]
use proconio::{
    fastout, input, input_interactive,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::line::LineSource,
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};

#[allow(unused_imports)]
use ac_library::{
    math,
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
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

#[allow(unused_imports)]
use std::io::{self, BufReader, StdinLock, Write};

#[allow(unused_imports)]
use rand::{prelude::*, rngs::StdRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};
#[allow(unused_imports)]
use rand_distr::{Distribution, Normal};
#[allow(unused_imports)]
use std::time::{Duration, Instant};

// v_i,j => (i,j) -> (i,j+1) vertical partition
// h_i,j => (i,j) -> (i+1,j) horizon partition
fn init_palette_partition(palette_size: usize) {
    // vertical partition
    for _i in 0..palette_size {
        for _j in 0..palette_size - 1 {
            print!("0 ");
        }
        println!();
    }

    // horizon partition
    for _i in 0..palette_size - 1 {
        for _j in 0..palette_size {
            print!("0 ");
        }
        println!();
    }
}

#[allow(dead_code)]
fn test_answer(make_color_count: usize) {
    for _i in 0..make_color_count {
        println!("1 1 1 0");
        println!("2 1 1")
    }
}

type CMY = (f64, f64, f64);
// type ColorDifference = f64;
type ColorDifferenceCost = f64;
type CommandSet = Vec<String>;

#[derive(Clone, Debug, PartialEq)]
struct PaletteWell {
    palette_size: usize,
    tube_type_count: usize,
    make_color_count: usize,
    max_turn_count: usize,
    color_cost: usize,
    my_color: Vec<CMY>,
    target_color: Vec<CMY>,
    current_turn_count: usize,
    color_set: Vec<usize>,
    anneal_time_secs: u64,
}

impl PaletteWell {
    fn new(
        palette_size: usize,
        tube_type_count: usize,
        make_color_count: usize,
        max_turn_count: usize,
        color_cost: usize,
        my_color: Vec<CMY>,
        target_color: Vec<CMY>,
        current_turn_count: usize,
        anneal_time_secs: u64,
    ) -> Self {
        let color_set = Vec::<usize>::new();
        PaletteWell {
            palette_size,
            tube_type_count,
            make_color_count,
            max_turn_count,
            color_cost,
            my_color,
            target_color,
            current_turn_count,
            color_set,
            anneal_time_secs,
        }
    }

    fn cost_cmy_manhattan_distance(&self, source: CMY, target: CMY) -> ColorDifferenceCost {
        let cost = 10000.0 * self.cmy_euclidean_distance(source, target);
        cost.round()
    }

    #[inline(always)]
    fn cmy_euclidean_distance(&self, source: CMY, target: CMY) -> f64 {
        self.least_squares_error(source, target).sqrt()
    }

    #[inline(always)]
    fn least_squares_error(&self, source: CMY, target: CMY) -> f64 {
        let da = source.0 - target.0;
        let db = source.1 - target.1;
        let dc = source.2 - target.2;
        da * da + db * db + dc * dc
    }

    fn best_cost(&self, target: CMY) -> (usize, ColorDifferenceCost) {
        let mut min_cost: ColorDifferenceCost = f64::MAX;
        let mut best_idx = 0;

        for (i, &tube) in self.my_color.iter().enumerate() {
            let cost = self.cost_cmy_manhattan_distance(tube, target);
            if cost < min_cost {
                min_cost = cost;
                best_idx = i;
            }
        }

        return (best_idx, min_cost);
    }

    fn best_cost_futuer_count(&self, start: usize, end: usize) -> (ColorDifferenceCost, CommandSet) {
        let mut cost = 0.0;
        let mut command: CommandSet = Vec::<String>::new();

        for i in start..end {
            let (idx, best_cost) = self.best_cost(self.target_color[i]);
            cost += best_cost;
            command.push(format!("1 0 0 {idx}"));
            command.push(format!("2 0 0"));
        }

        (cost, command)
    }

    fn random_cost_futuer_count(&mut self, start: usize, end: usize) -> (ColorDifferenceCost, CommandSet) {
        self.generate_color_set(end-start);
        //let cost = self.calc_cost_from_color_set(start, end);
        let (normal_cost, normal_command) = self.best_cost_futuer_count(start, end);
        let (cost, command) = self.simulated_annealing(start, end);

        if normal_cost < cost {
            let delta = normal_cost - cost;
            eprintln!("[DEBUG][IMPORTANT] delta={delta} normal_cost={normal_cost} cost={cost}");

            return (normal_cost, normal_command);
        }

        (cost, command)
    }

    fn simulated_annealing(&mut self, start: usize, end: usize) -> (ColorDifferenceCost, CommandSet) {
        let mut rng = rand::thread_rng();

        let start_t = Instant::now();
        let end_t = start_t + Duration::from_millis(self.anneal_time_secs);

        let mut best_cost = self.calc_cost_from_color_set(start, end);

        // let mut iter_count = 0;
        // let mut ttl_delta_cost = 0.0;

        let t0: f64 = 3.0; //9=80% 3=50%
        let t1: f64 = 0.4; //0.4=1% 0.3=0.1%
        #[allow(unused_assignments)]
        let mut temp = 0.0;
        // eprintln!("[DEBUG] START start={start} end={end} iter_count={iter_count}");

        while Instant::now() < end_t {
            // iter_count += 1;
            let now = Instant::now();
            let progress = (now - start_t).as_secs_f64() / (self.anneal_time_secs as f64/1000.0);
            if progress >= 1.0 {
                break;
            }
            temp = t0.powf(1.0 - progress) * t1.powf(progress);

            let backup_color_set = self.color_set.clone();

            self.change_color_set(temp);

            let new_cost = self.calc_cost_from_color_set(start, end);

            let delta_cost =  new_cost - best_cost;
            let per_once = delta_cost/(end-start) as f64;
            let p = f64::exp(-per_once / temp);

            // ttl_delta_cost += delta_cost/(end-start) as f64;
            // eprintln!("delta={delta_cost} perOnce={per_once} new_cost={new_cost} best_cost={best_cost}");

            if delta_cost < 0.0 {
                best_cost = new_cost;
            } else {
                if rng.gen::<f64>() >= p {
                    self.color_set = backup_color_set;
                } else {
                    // eprintln!("p={p} delta={delta_cost} temp={temp}");
                    best_cost = new_cost;
                }

            }
        }

        // eprintln!("[DEBUG] END start={start} end={end} iter_count={iter_count} ttl_delta_cost_ave={} temp={temp}", ttl_delta_cost/iter_count as f64);

        let mut result = Vec::<String>::new();
        for i in self.color_set.iter() {
            result.push(format!("1 0 0 {i}"));
        }
        for _ in start..end {
            result.push("2 0 0".to_string());
        }

        (best_cost, result)
    }

    fn change_color_set(&mut self, temp: f64) {
        let mut rng = rand::thread_rng();

        #[allow(unused_assignments)]
        let mut count = 3;
        if 2.0 < temp {
            count = 5;
        } else if 0.8 < temp {
            count = 5;
        } else {
            count = 5;
        }

        let mut order: Vec<usize> = (0..self.color_set.len()).collect();
        order.shuffle(&mut rng);

        for _ in 0..count {
            let change_idx = order.pop().unwrap();
            let change_tube = rng.gen_range(0..self.tube_type_count);

            self.color_set[change_idx] = change_tube;
        }
    }

    fn calc_cost_from_color_set(&self, start: usize, end: usize) -> ColorDifferenceCost {
        let mut cost =0.0;
        let source = self.generate_mixer_color();

        for i in start..end {
            cost += self.cost_cmy_manhattan_distance(source, self.target_color[i])
        }

        cost
    }

    fn generate_color_set(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        self.color_set = Vec::<usize>::new();

        for _ in 0..count {
            let random_idx: usize = rng.gen_range(0..self.tube_type_count);
            self.color_set.push(random_idx);
        }
    }

    fn generate_mixer_color(&self) -> CMY {
        let mut volume = 0.0;
        let mut mixer_color = (0.0, 0.0, 0.0);
        for i in self.color_set.clone() {
            mixer_color = self.mix_color(volume, 1.0, mixer_color, self.my_color[i]);
            volume += 1.0;
        }

        mixer_color
    }

    fn mix_color(&self, v_exists: f64, v_input: f64, cmy_exists: CMY, cmy_input: CMY) -> CMY {
        let c = (v_exists * cmy_exists.0 + v_input * cmy_input.0) / (v_exists + v_input);
        let m = (v_exists * cmy_exists.1 + v_input * cmy_input.1) / (v_exists + v_input);
        let y = (v_exists * cmy_exists.2 + v_input * cmy_input.2) / (v_exists + v_input);

        (c, m, y)
    }
}



trait TurnManager {
    fn countup_turn(&mut self);
    #[allow(dead_code)]
    fn current_turn(&self) -> usize;
    fn trun_left(&self) -> usize;
}

impl TurnManager for PaletteWell {
    #[inline(always)]
    fn countup_turn(&mut self) {
        self.current_turn_count += 1;
    }

    #[inline(always)]
    fn current_turn(&self) -> usize {
        self.current_turn_count
    }

    #[inline(always)]
    fn trun_left(&self) -> usize {
        self.max_turn_count - self.current_turn_count - 1
    }
}

fn mix_color(v_exists: f64, v_input: f64, cmy_exists: CMY, cmy_input: CMY) -> CMY {
    let c = (v_exists * cmy_exists.0 + v_input * cmy_input.0) / (v_exists + v_input);
    let m = (v_exists * cmy_exists.1 + v_input * cmy_input.1) / (v_exists + v_input);
    let y = (v_exists * cmy_exists.2 + v_input * cmy_input.2) / (v_exists + v_input);

    (c, m, y)
}

#[proconio::fastout]
fn main() {
    input! {
        palette_size: usize,
        tube_type_count: usize,
        make_color_count: usize,
        max_turn_count: usize,
        color_cost: usize,
        my_color: [(f64, f64, f64); tube_type_count],
        target_color: [(f64, f64, f64); make_color_count],
    }

    init_palette_partition(palette_size);
    //test_answer(make_color_count);

    let mut palette = PaletteWell::new(
        palette_size,
        tube_type_count,
        make_color_count,
        max_turn_count,
        color_cost,
        my_color,
        target_color,
        0,
        14
    );

    let target_list = palette.target_color.clone();
    let tube_list = palette.my_color.clone();

    let mut ttl_cost = 0.0;
    let mut volume = 0.0_f64;
    let mut chosen_color = (0.0, 0.0, 0.0);
    #[allow(unused_assignments)]
    let mut mixture_color = (0.0, 0.0, 0.0);

    let mut answer = Vec::<String>::new();

    for (tidx, &target) in target_list.iter().enumerate() {
        let mut min_cost: ColorDifferenceCost = f64::MAX;
        let mut best_idx = 0;

        for (i, &tube) in tube_list.iter().enumerate() {
            let cost = palette.cost_cmy_manhattan_distance(tube, target);
            if cost < min_cost {
                min_cost = cost;
                best_idx = i;
            }
        }
        mixture_color = palette.my_color[best_idx];

        if volume > 0.0 {
            let cost = palette.cost_cmy_manhattan_distance(chosen_color, target);
            if cost < min_cost + palette.color_cost as f64 {
                min_cost = cost;
                mixture_color = chosen_color;
            } else {
                for _ in 0..volume.clone() as usize {
                    answer.push("3 0 0".to_string());
                    volume -= 1.0;
                    palette.countup_turn();
                    // eprintln!("volume={volume}");
                }
                answer.push(format!("1 0 0 {best_idx}"));
                palette.countup_turn();
                volume += 1.0;
                // eprintln!("volume={volume}");
            }
        } else {
            answer.push(format!("1 0 0 {best_idx}"));
            palette.countup_turn();
            volume += 1.0;
            // eprintln!("volume={volume}");
        }

        let mut current_cost = min_cost;
        let need_turn = (palette.make_color_count - tidx) * 2;

        // eprintln!(
        //     "[DEBUG] target={} start tube={} cost={} (color_cost={}) volume={} turn={} (max_turn={} turn_left={} need={})",
        //     tidx,
        //     best_idx,
        //     current_cost,
        //     palette.color_cost,
        //     volume,
        //     palette.current_turn(),
        //     palette.max_turn_count,
        //     palette.trun_left(),
        //     need_turn + volume as usize
        // );

        if 1 + need_turn + volume as usize > palette.trun_left() {
            ttl_cost += current_cost;
            answer.push("2 0 0".to_string());
            volume -= 1.0;
            palette.countup_turn();

            // eprintln!(
            //     "[ERROR] ****** NEED TO FINISH ****** turn_left={} need_turn={}",
            //     palette.trun_left(),
            //     need_turn
            // );
        } else {
            loop {
                let mut best_gain = 0.0_f64;
                let mut best_next_gain = 0.0_f64;

                let mut chosen_idx = None;
                let mut next_chosen_idx = None;

                let mut next_chosen_color = (0.0, 0.0, 0.0);
                chosen_color = mixture_color;

                let mut chosen_cost = current_cost;
                let mut next_chosen_cost = current_cost;

                let special_flag = false;

                for (i, &candidate) in palette.my_color.clone().iter().enumerate() {
                    let new_color = mix_color(volume, 1.0, mixture_color, candidate);
                    let new_cost = palette.cost_cmy_manhattan_distance(new_color, target);
                    let gain = current_cost - new_cost;

                    if tidx < 999 && volume == 1.0 {
                        let next_target = palette.clone().target_color[tidx + 1];
                        let next_cost = palette.cost_cmy_manhattan_distance(new_color, next_target);
                        let next_best_cost = palette.best_cost(next_target);

                        let check_cost = (next_best_cost.1 - next_cost) + gain;

                        // if check_cost > 0.0 {
                        //     eprintln!("[BEST] current={current_cost} new={new_cost} gain={gain} cc={check_cost} next={next_cost} next_best={} color_cost={}", next_best_cost.1, palette.color_cost);
                        // }

                        if check_cost > best_next_gain {
                            best_next_gain = check_cost;
                            next_chosen_idx = Some(i);
                            next_chosen_color = new_color;
                            next_chosen_cost = new_cost;

                            //if check_cost > current_cost { special_flag = true; }
                        }
                    }

                    if gain > best_gain {
                        best_gain = gain;
                        chosen_idx = Some(i);
                        chosen_color = new_color;
                        chosen_cost = new_cost;
                    }
                }

                // clean up
                if volume >= 40.0 {
                    // eprintln!("[DEBUG] cleanup");
                    for _ in 2..volume.clone() as usize {
                        answer.push("3 0 0".to_string());
                        volume -= 1.0;
                        palette.countup_turn();
                    }
                }

                if 1 + need_turn + volume as usize > palette.trun_left() {
                    if let Some(idx) = chosen_idx {
                        answer.push(format!("1 0 0 {idx}"));
                        palette.countup_turn();
                        volume += 1.0;

                        current_cost = chosen_cost;

                        // eprintln!(
                        //     "[DEBUG] target={} add tube={} gain={:.3} new_cost={:.3} volume={} turn={} (turn_left={} need={})",
                        //     tidx, idx, best_gain, current_cost, volume, palette.current_turn(), palette.trun_left(), need_turn + volume as usize
                        // );
                    }

                    ttl_cost += current_cost;
                    answer.push("2 0 0".to_string());
                    palette.countup_turn();
                    volume -= 1.0;

                    // eprintln!(
                    //     "[ERROR] ****** NEED TO FINISH ****** volume={}, turn_left={} need={}",
                    //     volume,
                    //     palette.trun_left(),
                    //     need_turn + volume as usize
                    // );

                    // let volume_bool = volume > 40.0;
                    // eprintln!("[DEBUG] volume={volume} capacity={volume_bool}");

                    for _ in 0..volume.clone() as usize {
                        answer.push("3 0 0".to_string());
                        volume -= 1.0;
                        palette.countup_turn();
                        // eprintln!(
                        //     "volume={}, turn_left={} need={}",
                        //     volume,
                        //     palette.trun_left(),
                        //     need_turn + volume as usize
                        // );
                    }

                    break;
                } else if special_flag {
                    //
                    let idx = next_chosen_idx.unwrap();

                    answer.push(format!("1 0 0 {idx}"));
                    palette.countup_turn();
                    volume += 1.0;

                    chosen_color = next_chosen_color;
                    current_cost = next_chosen_cost;

                    // eprintln!(
                    //     "[BEST][DEBUG] target={} add tube={} gain={:.3} new_cost={:.3} volume={} turn={} (turn_left={} need={})",
                    //     tidx, idx, best_gain, current_cost, volume, palette.current_turn(), palette.trun_left(), need_turn
                    // );

                    ttl_cost += current_cost;
                    answer.push("2 0 0".to_string());
                    palette.countup_turn();
                    volume -= 1.0;

                    break;

                } else if best_gain > palette.color_cost as f64 {
                    let idx = chosen_idx.unwrap();

                    answer.push(format!("1 0 0 {idx}"));
                    palette.countup_turn();
                    volume += 1.0;

                    mixture_color = chosen_color;
                    current_cost = chosen_cost;

                    // eprintln!(
                    //     "[DEBUG] target={} add tube={} gain={:.3} new_cost={:.3} volume={} turn={} (turn_left={} need={})",
                    //     tidx, idx, best_gain, current_cost, volume, palette.current_turn(), palette.trun_left(), need_turn
                    // );
                } else if !next_chosen_idx.is_none() {
                    //
                    let idx = next_chosen_idx.unwrap();

                    answer.push(format!("1 0 0 {idx}"));
                    palette.countup_turn();
                    volume += 1.0;

                    chosen_color = next_chosen_color;
                    current_cost = next_chosen_cost;

                    // eprintln!(
                    //     "[BEST][DEBUG] target={} add tube={} cc={} gain={:.3} new_cost={:.3} volume={} turn={} (turn_left={} need={})",
                    //     tidx, idx, best_next_gain, best_gain, current_cost, volume, palette.current_turn(), palette.trun_left(), need_turn
                    // );

                    ttl_cost += current_cost;
                    answer.push("2 0 0".to_string());
                    palette.countup_turn();
                    volume -= 1.0;

                    break;

                } else {
                    ttl_cost += current_cost;
                    answer.push("2 0 0".to_string());
                    palette.countup_turn();
                    volume -= 1.0;

                    // let volume_bool = volume > 40.0;
                    // eprintln!("[DEBUG] volume={volume} capacity={volume_bool}");

                    break;
                }
            }
        }
    }

    // let (tmp, _) = palette.best_cost_futuer_count(0, 1000);

    let chunk_size = 5;

    let mut random_ttl_cost = 0.0;
    let mut all_commands = Vec::new();

    for start in (0..palette.make_color_count).step_by(chunk_size) {
        let end = (start + chunk_size).min(palette.make_color_count);
        let (cost, cmds) = palette.random_cost_futuer_count(start, end);
        random_ttl_cost += cost;
        all_commands.extend(cmds);
    }

    if random_ttl_cost < ttl_cost {
        for result in all_commands {
            println!("{}", result);
        }
    } else {
        for result in answer {
            println!("{}", result);
        }
    }

    // eprintln!("default={tmp}");
    eprintln!("total_cost={ttl_cost}");
    eprintln!("random1000cases={random_ttl_cost}");
}
