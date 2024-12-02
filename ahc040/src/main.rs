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
use rand::{Rng, thread_rng};
#[allow(unused_imports)]
use rand_distr::{Normal, Distribution};

fn main() {
    input_interactive! {
        n: usize,
        t: usize,
        sigma: i64,
        rects: [(i64, i64); n],
    }

    let mut rng = thread_rng();

    let rectangles: Vec<(usize, i64, i64)> = rects
        .into_iter()
        .enumerate()
        .map(|(i, (w, h))| (i, w, h))
        .collect();

    // T Count
    let mut count = 1;

    // Split rects list
    let mut split_count = 4;
    if 50 < n {
        split_count = n / 10 - 1;
    }

    let mut split = (n + split_count - 1) / split_count;
    let mut split_point: Vec<usize> = (0..split_count).map(|i| i * split).collect();

    let mut max_offset = split / 2;
    let mut adjustment_step: isize = 1;

    loop {
        if t < count {
            break;
        }

        // Special First time
        println!("# Cycle T={}", count);
        println!("# Split count={}", split_count);
        println!("# Adjust step={}", adjustment_step);
        println!("# Split={:?}", split_point);

        // Save data for rectangle
        let mut placement = Vec::new();

        // Update max width
        let mut current_x = 0;
        let mut max_width = 0;
        let mut first_time = true;
        let mut split_xxx = 0;
        // Update max height
        let mut max_height = 0;

        let split_point_backup = split_point.clone();

        for &(idx, w, h) in &rectangles {
            let mut rotate = 0;
            let (mut w, mut h) = (w, h);

            // Rotate
            if w > h {
                rotate = 1;
                std::mem::swap(&mut w, &mut h);
            }

            // Add placement
            let bi = if split_point.contains(&idx) {
                split_xxx += 1;
                -1
            } else {
                (idx - 1) as i64
            };

            let mean = 0.0;
            let normal = Normal::new(mean, sigma as f64).unwrap();

            let random_number = normal.sample(&mut rng);

            // Update x
            current_x += w + random_number as i64;

            if bi == -1 && first_time && idx != 0 {
                first_time = false;
                max_width = current_x;
                println!("# idx={} maxwidth={}", idx, max_width);
            }

            let bi = if !first_time && max_width <= current_x && split_xxx < split_point.len() && !split_point.contains(&idx) {
                println!("# idx={} split_idx={}", &idx, split_xxx);
                split_point[split_xxx] = 0;
                split_xxx += 1;
                println!("# Split={:?}", split_point);
                current_x = 0;
                -1
            } else if split_point.contains(&idx) {
                println!("# idx={}", idx);
                if max_width < current_x {
                    println!("# idx={} maxwidth={}", idx, max_width);
                    max_width = current_x;
                }
                -1
            } else {
                (idx - 1) as i64
            };

            if bi == -1 {
                current_x = 0;
            }

            // println!("# Width={}", current_x);

            placement.push((idx, rotate, 'U', bi));


            // Update max height
            if h > max_height {
                max_height = h;
            }
        }

        // reset
        split_point = split_point_backup;

        println!("{}", placement.len());
        for (p, r, d, b) in &placement {
            println!("{} {} {} {}", p, r, d, b);
        }
        stdout().flush().unwrap();

        input_interactive! {
            _w_prime: i64,
            _h_prime: i64,
        }

        count += 1;

        for indices in (1..split_count).combinations(adjustment_step as usize) {
            for adjustment in [-adjustment_step, adjustment_step] {
                if t < count {
                    break;
                }

                // Update max width
                let mut current_x = 0;
                let mut max_width = 0;
                let mut first_time = true;
                let mut split_xxx = 0;

                println!("# Cycle T={}", count);
                println!("# Split count={}", split_count);
                println!("# Adjust step={}", adjustment_step);
                for &idx in &indices {
                    split_point[idx] = (split_point[idx] as isize + adjustment) as usize;
                }
                println!("# Split={:?}", split_point);
                let split_point_backup = split_point.clone();

                // Save data for rectangle
                let mut placement = Vec::new();

                // Update x
                // let mut current_x = 0;
                // Update max height
                let mut max_height = 0;

                for &(idx, w, h) in &rectangles {
                    let mut rotate = 0;
                    let (mut w, mut h) = (w, h);

                    // Rotate
                    if w > h {
                        rotate = 1;
                        std::mem::swap(&mut w, &mut h);
                    }

                    // Add placement
                    let bi = if split_point.contains(&idx) {
                        split_xxx += 1;
                        -1
                    } else {
                        (idx - 1) as i64
                    };

                    let mean = 0.0;
                    let normal = Normal::new(mean, sigma as f64).unwrap();

                    let random_number = normal.sample(&mut rng);

                    current_x += w + random_number as i64;

                    if bi == -1 && first_time && idx != 0 {
                        first_time = false;
                        max_width = current_x;
                        println!("# idx={} maxwidth={}", idx, max_width);
                    }

                    let bi = if !first_time && max_width <= current_x && split_xxx < split_point.len() && !split_point.contains(&idx) {
                        println!("# idx={} split_idx={}", &idx, split_xxx);
                        split_point[split_xxx] = 0;
                        split_xxx += 1;
                        println!("# Split={:?}", split_point);
                        current_x = 0;
                        -1
                    } else if split_point.contains(&idx) {
                        println!("# idx={}", idx);
                        if max_width < current_x {
                            println!("# idx={} maxwidth={}", idx, max_width);
                            max_width = current_x;
                        }
                        -1
                    } else {
                        (idx - 1) as i64
                    };

                    if bi == -1 {
                        current_x = 0;
                    }

                    // println!("# Width={}", current_x);

                    placement.push((idx, rotate, 'U', bi));

                    // Update x
                    // current_x += w;

                    // Update max height
                    if h > max_height {
                        max_height = h;
                    }
                }

                println!("{}", placement.len());
                for (p, r, d, b) in &placement {
                    println!("{} {} {} {}", p, r, d, b);
                }
                stdout().flush().unwrap();

                input_interactive! {
                    _w_prime: i64,
                    _h_prime: i64,
                }

                // reset
                split_point = split_point_backup;

                for &idx in &indices {
                    split_point[idx] = (split_point[idx] as isize - adjustment) as usize;
                }

                count += 1;
                if t < count {
                    break;
                }
            }
            if t < count {
                break;
            }
        }

        if t < count {
            break;
        }

        adjustment_step += 1;
        if adjustment_step as usize > max_offset {
            split_count += 1;
            split = (n + split_count - 1) / split_count;
            split_point = (0..split_count).map(|i| i * split).collect();

            max_offset = (split / 2) + 1;
            adjustment_step = 1;
        }
    }

    if count < t {
        for _ in count..=t {
            println!("# Cycle T={}", count);
            println!("1");
            println!("0 0 U -1");
            stdout().flush().unwrap();

            input_interactive! {
                _w_prime: i64,
                _h_prime: i64,
            }

            count += 1;
        }
    }

}
