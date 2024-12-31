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

struct PackingStatus {
    count: usize,
    split_point: Vec<usize>,
    split_count: usize,
}
struct Rectangle {
    list: Vec<(usize, usize, (i64, i64))>,
}

struct Container {
    status: PackingStatus,
    rects: Rectangle,
}

trait Packing {
    fn new(items: Vec<(i64, i64)>) -> Self;
    fn check_rotate(&mut self);
    fn apply_margin(&mut self, sigma: i64);
    fn find_split_point(&mut self);
}

impl Packing for Container {
    fn new(items: Vec<(i64, i64)>) -> Self {
        Container {
            status: PackingStatus {
                count: 1,
                split_point: vec![0],
                split_count: 0,
            },
            rects: Rectangle {
                list: items
                    .into_iter()
                    .enumerate()
                    .map(|(i, (w, h))| (i, 0, (w, h)))
                    .collect()
            },
        }
    }

    fn check_rotate(&mut self) {
        for item in &mut self.rects.list {
            let (_, rotate, (w, h)) = item;
            if *h+2000 < *w {
                *rotate = 1;
                std::mem::swap(w, h);
            }
        }
    }

    fn apply_margin(&mut self, sigma: i64) {

        let mean = 0.0;
        let mut rng = thread_rng();

        for item in &mut self.rects.list {
            let (_, _, (w, h)) = item;
            let normal = Normal::new(mean, sigma as f64).unwrap();

            let width_padding = normal.sample(&mut rng) as i64;
            let height_padding = normal.sample(&mut rng) as i64;

            *w = *w + width_padding;
            *h = *h + height_padding;
        }

    }

    fn find_split_point(&mut self) {
        let n = self.rects.list.len();

        let mut sum_width_left = vec![0; n];
        let mut max_height_left = vec![0; n];
        let mut width_accum = 0;
        let mut height_max = 0;

        for i in 0..n {
            let (_, _, (width, height)) = self.rects.list[i];
            width_accum += width;
            height_max = height_max.max(height);
            sum_width_left[i] = width_accum;
            max_height_left[i] = height_max;
        }

        let mut sum_width_right = vec![0; n];
        let mut max_height_right = vec![0; n];
        width_accum = 0;
        height_max = 0;

        for i in (0..n).rev() {
            let (_, _, (width, height)) = self.rects.list[i];
            width_accum += width;
            height_max = height_max.max(height);
            sum_width_right[i] = width_accum;
            max_height_right[i] = height_max;
        }

        self.status.split_point = (0..n - 1)
            .filter(|&i| {
                sum_width_left[i] == max_height_left[i] && sum_width_right[i + 1] == max_height_right[i + 1]
            })
            .collect();

        if self.status.split_point.is_empty() {
            let mut split_count = 4;
            if 50 < n {
                split_count = (n / 10) - 1;
            }

            let split = (n + split_count - 1) / split_count;

            self.status.split_point = (0..split_count).map(|i| i * split).collect();
        }

        self.status.split_count = self.status.split_point.len();
    }
}

fn main() {
    input_interactive! {
        n: usize,
        t: usize,
        sigma: i64,
        rects: [(i64, i64); n],
    }

    let mut container = Container::new(rects);

    let mut max_offset = 2;
    let mut adjustment_step: isize = 1;

    //container.apply_margin(sigma);
    container.check_rotate();
    container.find_split_point();

    loop {
        if t < container.status.count {
            break;
        }

        // Special First time
        println!("# Cycle T={}", container.status.count);
        println!("# Split count={}", container.status.split_count);
        println!("# Adjust step={}", adjustment_step);
        println!("# Split={:?}", container.status.split_point);

        // Save data for rectangle
        let mut placement = Vec::new();

        // Update max width
        let mut current_x = 0;
        let mut max_width = 0;
        let mut first_time = true;
        let mut split_xxx = 0;
        // Update max height
        let mut max_height = 0;

        let split_point_backup = container.status.split_point.clone();

        for &(idx, rotate, (w, h)) in &container.rects.list {
            let (w, h) = (w, h);

            // Add placement
            let bi = if container.status.split_point.contains(&idx) {
                split_xxx += 1;
                -1
            } else {
                (idx - 1) as i64
            };

            current_x += w;

            if bi == -1 && first_time && idx != 0 {
                first_time = false;
                max_width = current_x;
                println!("# idx={} maxwidth={}", idx, max_width);
            }

            let bi = if !first_time && max_width+sigma <= current_x && split_xxx < container.status.split_point.len() && !container.status.split_point.contains(&idx) {
                println!("# idx={} split_idx={}", &idx, split_xxx);
                container.status.split_point[split_xxx] = 0;
                split_xxx += 1;
                println!("# Split={:?}", container.status.split_point);
                max_width = current_x;
                current_x = 0;
                -1
            } /* else if container.status.split_point.contains(&idx) {
                println!("# idx={}", idx);
                if max_width < current_x {
                    println!("# idx={} maxwidth={}", idx, max_width);
                    max_width = current_x;
                }
                -1
            } */ else {
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
        container.status.split_point = split_point_backup;

        println!("{}", placement.len());
        for (p, r, d, b) in &placement {
            println!("{} {} {} {}", p, r, d, b);
        }
        stdout().flush().unwrap();

        input_interactive! {
            _w_prime: i64,
            _h_prime: i64,
        }

        container.status.count += 1;

        for indices in (1..container.status.split_count).combinations(adjustment_step as usize) {
            for adjustment in [-adjustment_step, adjustment_step] {
                if t < container.status.count {
                    break;
                }

                // Update max width
                let mut current_x = 0;
                let mut max_width = 0;
                let mut first_time = true;
                let mut split_xxx = 0;

                println!("# Cycle T={}", container.status.count);
                println!("# Split count={}", container.status.split_count);
                println!("# Adjust step={}", adjustment_step);
                for &idx in &indices {
                    container.status.split_point[idx] = (container.status.split_point[idx] as isize + adjustment) as usize;
                }
                println!("# Split={:?}", container.status.split_point);
                let split_point_backup = container.status.split_point.clone();

                // Save data for rectangle
                let mut placement = Vec::new();

                // Update x
                // let mut current_x = 0;
                // Update max height
                let mut max_height = 0;

                for &(idx, rotate, (w, h)) in &container.rects.list {
                    let (w, h) = (w, h);

                    // Add placement
                    let bi = if container.status.split_point.contains(&idx) {
                        split_xxx += 1;
                        -1
                    } else {
                        (idx - 1) as i64
                    };

                    current_x += w;

                    if bi == -1 && first_time && idx != 0 {
                        first_time = false;
                        max_width = current_x;
                        println!("# idx={} maxwidth={}", idx, max_width);
                    }

                    let bi = if !first_time && max_width <= current_x && split_xxx < container.status.split_point.len() && !container.status.split_point.contains(&idx) {
                        println!("# idx={} split_idx={}", &idx, split_xxx);
                        container.status.split_point[split_xxx] = 0;
                        split_xxx += 1;
                        println!("# Split={:?}", container.status.split_point);
                        current_x = 0;
                        -1
                    } else if container.status.split_point.contains(&idx) {
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
                container.status.split_point = split_point_backup;

                for &idx in &indices {
                    container.status.split_point[idx] = (container.status.split_point[idx] as isize - adjustment) as usize;
                }

                container.status.count += 1;
                if t < container.status.count {
                    break;
                }
            }
            if t < container.status.count {
                break;
            }
        }

        if t < container.status.count {
            break;
        }

        adjustment_step += 1;
        if adjustment_step as usize > max_offset {
            container.status.split_count += 1;
            let split = (n + container.status.split_count - 1) / container.status.split_count;
            container.status.split_point = (0..container.status.split_count).map(|i| i * split).collect();

            max_offset = (split / 2) + 1;
            adjustment_step = 1;
        }
    }

    if container.status.count < t {
        for _ in container.status.count..=t {
            println!("# Cycle T={}", container.status.count);
            println!("1");
            println!("0 0 U -1");
            stdout().flush().unwrap();

            input_interactive! {
                _w_prime: i64,
                _h_prime: i64,
            }

            container.status.count += 1;
        }
    }

}

