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
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    math,
    Dsu,
    // new(size: usize) -> Self
    // merge(&mut self, a: usize, b: usize) -> usize
    // same(&mut self, a: usize, b: usize) -> bool
    // leader(&mut self, a: usize) -> usize
    // size(&mut self, a: usize) -> usize
    // groups(&mut self) -> Vec<Vec<usize>>
    FenwickTree,
    Max,
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&mut self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree,
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

#[allow(unused_imports)]
use std::io::{stdout, Write};

#[allow(unused_imports)]
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
#[allow(unused_imports)]
use rand_distr::{Distribution, Normal};
#[allow(unused_imports)]
use std::time::{Duration, Instant};

type Pos = (usize, usize);
type HomePos = (usize, usize);
type OfficePos = (usize, usize);
type ManhattanDistance = usize;
type HomeOfficePos = (HomePos, OfficePos, ManhattanDistance);
type HomeOfficePosList = Vec<HomeOfficePos>;
type StationPos = (i32, usize, usize);
type StationPosList = Vec<StationPos>;

const STATION_COST: usize = 5000;
const RAIL_COST: usize = 100;
const EXTEND_DIST: i32 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Track,
    Station,
}

struct Railway {
    home_office: HomeOfficePosList,
    station_candidates: StationPosList,
    capital: i64,
    t: usize,
    n: usize,
    grid: Vec<Vec<Cell>>,
    path: Vec<Pos>,
    stations: Vec<Pos>,
    loop_count: usize,
    least_dist: usize,
}

impl Railway {
    fn new(n: usize, m: usize, k: i64, t: usize) -> Self {
        Railway {
            home_office: Vec::with_capacity(m),
            station_candidates: Vec::new(),
            capital: k,
            t: t,
            n: n,
            grid: vec![vec![Cell::Empty; n]; n],
            path: Vec::new(),
            stations: Vec::new(),
            loop_count: 0,
            least_dist: 20,
        }
    }

    fn calc_revenue(&self) -> i64 {
        let mut revenue = 0;
        for &((home_r, home_c), (office_r, office_c), dist) in self.home_office.iter() {
            if self.is_covered(home_r, home_c) && self.is_covered(office_r, office_c) {
                revenue += dist as i64;
            }
        }
        revenue
    }

    fn calc_candidate_revenue(&self, new_station: Pos) -> i64 {
        let (new_r, new_c) = new_station;
        let mut additional_revenue = 0;
        for &((home_r, home_c), (office_r, office_c), dist) in self.home_office.iter() {
            let current_home_covered = self.is_covered(home_r, home_c);
            let current_office_covered = self.is_covered(office_r, office_c);

            let candidate_home_covered =
                current_home_covered || Railway::cell_in_range(home_r, home_c, new_r, new_c);
            let candidate_office_covered =
                current_office_covered || Railway::cell_in_range(office_r, office_c, new_r, new_c);

            if candidate_home_covered
                && candidate_office_covered
                && !(current_home_covered && current_office_covered)
            {
                additional_revenue += dist as i64;
            }
        }
        additional_revenue
    }

    fn cell_in_range(r: usize, c: usize, station_r: usize, station_c: usize) -> bool {
        let dr = (r as i64 - station_r as i64).abs();
        let dc = (c as i64 - station_c as i64).abs();
        dr + dc <= 2
    }

    // Is station exists from (r, c) - Manhattan <= 2
    fn is_covered(&self, r: usize, c: usize) -> bool {
        for dr in -2i32..=2 {
            for dc in -2i32..=2 {
                if dr.abs() + dc.abs() <= 2 {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < self.n as i32 && nc >= 0 && nc < self.n as i32 {
                        if self.grid[nr as usize][nc as usize] == Cell::Station {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    // Check station can construct at candidate position
    fn can_place_station(&self, candidate: Pos) -> bool {
        let (r, c) = candidate;
        for dr in -EXTEND_DIST..=EXTEND_DIST {
            for dc in -EXTEND_DIST..=EXTEND_DIST {
                if dr.abs() + dc.abs() <= EXTEND_DIST {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < self.n as i32 && nc >= 0 && nc < self.n as i32 {
                        if self.grid[nr as usize][nc as usize] == Cell::Station {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    // Find station candidate
    // (Number of home/office, r, c), sort by dest
    fn create_station_candidate(&mut self) {
        for r in 0..self.n {
            for c in 0..self.n {
                let mut cover_count = 0;
                for ((is, js), (it, jt), _distance) in self.home_office.clone() {
                    let d_s = (is as i64 - r as i64).abs() + (js as i64 - c as i64).abs();
                    let d_t = (it as i64 - r as i64).abs() + (jt as i64 - c as i64).abs();
                    if d_s <= 2 || d_t <= 2 {
                        cover_count += 1;
                    }
                }
                self.station_candidates.push((cover_count, r, c));
            }
        }
        // Sorted by number of home/station
        self.station_candidates.sort_by_key(|x| x.0);
        self.station_candidates.reverse();
        println!("# Candidate Station {}", self.station_candidates.len());
    }

    // Find candidate station from home or office
    fn find_first_investigate(&mut self, first_time: bool) -> bool {
        for (count, r, c) in self.station_candidates.clone() {
            println!("# Start pos=({}, {}) Station", r, c);
            match self.find_home_or_office(r, c) {
                Some(result) => {
                    let mut output = Vec::new();
                    for ((i, j), _d) in result {
                        // println!("# pos=({}, {}) dist={}", i, j, d);
                        let next_station_pos = self
                            .find_candidate_station_from_pos(i, j)
                            .unwrap_or((1, i, j));

                        // println!("# pos=({}, {}) count={}", next_station_pos.1, next_station_pos.2, next_station_pos.0);
                        output.push(next_station_pos);
                    }
                    output.sort_by_key(|x| x.0);
                    output.reverse();

                    output.dedup_by(|a, b| a.1 == b.1 && a.2 == b.2);

                    for (d, i, j) in output {
                        println!("# Goal pos=({}, {}) count={} Station", i, j, d);

                        self.path = self.bfs_path(r, c, i, j);

                        if self.path.is_empty() {
                            println!("# Route not found");
                            continue;
                        }

                        let total_cost = (self.path.len() * RAIL_COST) + (2 * STATION_COST);

                        if self.is_new_station_pair_investment_profitable(
                            (r, c),
                            (i, j),
                            self.path.len(),
                        ) {
                            // Cost effectiveness
                            println!("# is_investigate true from=({} {}) to=({}, {})", r, c, i, j);
                        } else {
                            println!("# is_investigate false from=({} {}) to({}, {})", r, c, i, j);
                            self.path = Vec::new();
                            continue;
                        }

                        if self.can_place_station((r, c)) && self.can_place_station((i, j)) {
                            println!(
                                "# can_place_station true from=({}, {}) to=({}, {})",
                                r, c, i, j
                            );
                        } else {
                            println!(
                                "# can_place_station false from=({}, {}) to=({}, {})",
                                r, c, i, j
                            );
                            self.path = Vec::new();
                            continue;
                        }

                        if self.capital < total_cost as i64 && first_time {
                            println!(
                                "# Capital cost over... continue capital={} total_cost={}",
                                self.capital, total_cost
                            );
                            self.path = Vec::new();
                            continue;
                        }

                        // This path works
                        self.stations.push((r, c));
                        self.stations.push((i, j));

                        self.station_candidates.retain(|&(cnt, rr, cc)| {
                            !((cnt == count && rr == r && cc == c)
                                || (cnt == d && rr == i && cc == j))
                        });

                        println!("# This path works");
                        println!("# Station {:?}", self.stations);
                        println!("# Route {:?}", self.path);

                        return true;
                    }
                }
                None => {
                    println!("# Worng?");
                }
            }
            break;
        }
        return false;
    }

    // Find home or station from (r, c)
    fn find_home_or_office(&self, r: usize, c: usize) -> Option<Vec<(Pos, ManhattanDistance)>> {
        let mut output = Vec::new();
        for &((is, js), (it, jt), dist) in self.home_office.iter() {
            let d_s = (is as i64 - r as i64).abs() + (js as i64 - c as i64).abs();
            let d_t = (it as i64 - r as i64).abs() + (jt as i64 - c as i64).abs();
            if d_s <= 2 && self.least_dist < dist {
                output.push(((it, jt), dist));
            } else if d_t <= 2 && self.least_dist < dist {
                output.push(((is, js), dist));
            }
        }

        if output.len() == 0 {
            None
        } else {
            output.sort_by_key(|x| x.1);
            output.reverse();
            Some(output)
        }
    }

    // Find station from (r, c)
    fn find_candidate_station_from_pos(&self, r: usize, c: usize) -> Option<StationPos> {
        for &(count, rr, cc) in self.station_candidates.iter() {
            let d = (rr as i64 - r as i64).abs() + (cc as i64 - c as i64).abs();
            if d <= 2 {
                return Some((count, rr, cc));
            }
        }

        None
    }

    // Find route for rail construction
    fn bfs_path(&self, sr: usize, sc: usize, tr: usize, tc: usize) -> Vec<Pos> {
        let mut visited = vec![vec![false; self.n]; self.n];
        let mut parent = vec![vec![None; self.n]; self.n];
        let mut queue = VecDeque::new();

        visited[sr][sc] = true;
        queue.push_back((sr, sc));

        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        while let Some((r, c)) = queue.pop_front() {
            if r == tr && c == tc {
                let mut path = Vec::new();
                let mut cur = (r, c);
                while let Some(prev) = parent[cur.0][cur.1] {
                    path.push(cur);
                    cur = prev;
                }
                path.push(cur);
                path.reverse();
                return path;
            }
            for (dr, dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= self.n as i32 || nc < 0 || nc >= self.n as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if visited[nr][nc] {
                    continue;
                }

                if self.grid[nr][nc] == Cell::Track || self.grid[nr][nc] == Cell::Station {
                    continue;
                }
                visited[nr][nc] = true;
                parent[nr][nc] = Some((r, c));
                queue.push_back((nr, nc));
            }
        }

        Vec::new()
    }

    fn extend_network(&mut self) -> bool {
        let mut built_stations = Vec::new();
        for r in 0..self.n {
            for c in 0..self.n {
                if self.grid[r][c] == Cell::Station {
                    built_stations.push((r, c));
                }
            }
        }

        if let Some(((r, c), (cand_r, cand_c), find_path, cost)) = self.find_best_extension() {
            self.path = find_path;
            if self.path.is_empty() {
                // Route not found
                self.path = Vec::new();
                return false;
            }

            if self.can_place_station((cand_r, cand_c)) {
                println!("# can_place_station true ({}, {})", cand_r, cand_c);
            } else {
                println!("# can_place_station false ({}, {})", cand_r, cand_c);
                self.path = Vec::new();
                return false;
            }

            if self.is_investigate((cand_r, cand_c), self.path.len()) {
                // Cost effectiveness
                println!("# is_investigate true ({}, {})", cand_r, cand_c);
            } else {
                println!("# is_investigate false ({}, {})", cand_r, cand_c);
                self.path = Vec::new();
                return false;
            }

            // This path works
            self.stations.push((r, c));
            self.stations.push((cand_r, cand_c));

            self.station_candidates
                .retain(|&(_cnt, rr, cc)| !((rr == cand_r) && (cc == cand_c)));

            println!("# This path works");
            println!("# Cost={}", cost);
            println!("# Station {:?}", self.stations);
            println!("# Route {:?}", self.path);

            return true;
        }

        false
    }

    fn find_best_extension(&self) -> Option<(Pos, Pos, Vec<Pos>, i64)> {
        let mut best_candidate: Option<(Pos, Pos, Vec<Pos>, i64)> = None;

        let mut built_stations = Vec::new();
        for r in 0..self.n {
            for c in 0..self.n {
                if self.grid[r][c] == Cell::Station {
                    built_stations.push((r, c));
                }
            }
        }

        for &(src_r, src_c) in &built_stations {
            if let Some(home_office_list) = self.find_home_or_office(src_r, src_c) {
                for &((dest_r, dest_c), _dist) in home_office_list.iter() {
                    if let Some((_count, cand_r, cand_c)) =
                        self.find_candidate_station_from_pos(dest_r, dest_c)
                    {
                        let new_station = (cand_r, cand_c);

                        if !self.can_place_station(new_station) {
                            continue;
                        }

                        let path = self.bfs_path(src_r, src_c, cand_r, cand_c);
                        if path.is_empty() {
                            continue;
                        }

                        let cost = (path.len() * RAIL_COST) as i64 + STATION_COST as i64;

                        if !self.is_new_station_pair_investment_profitable(
                            (src_r, src_c),
                            new_station,
                            path.len(),
                        ) {
                            continue;
                        }

                        if best_candidate.is_none() || cost < best_candidate.as_ref().unwrap().3 {
                            best_candidate =
                                Some(((src_r, src_c), new_station, path.clone(), cost));
                        }
                    }
                }
            }
        }
        best_candidate
    }

    fn is_investigate(&self, new_station: Pos, path_len: usize) -> bool {
        let revenue: i64 = self.calc_candidate_revenue(new_station);
        let station_construct = 1;
        let total_revenue: i64 =
            revenue * (self.t - self.loop_count - path_len - station_construct) as i64;
        let diff_revenue: i64 = total_revenue - STATION_COST as i64 - (path_len * RAIL_COST) as i64;

        if diff_revenue < 1 {
            return false;
        }

        println!(
            "# loop_count={} revenue={} diff_revenue={}",
            self.loop_count, revenue, diff_revenue
        );

        true
    }

    fn is_station(&self) -> bool {
        let (r, c) = self.stations[0].clone();
        if self.grid[r][c] == Cell::Station {
            return true;
        }
        false
    }

    fn calc_pair_revenue(&self, station1: Pos, station2: Pos) -> i64 {
        let mut revenue = 0;
        for &((home_r, home_c), (office_r, office_c), dist) in self.home_office.iter() {
            let home_covered = Railway::cell_in_range(home_r, home_c, station1.0, station1.1)
                || Railway::cell_in_range(home_r, home_c, station2.0, station2.1);
            let office_covered = Railway::cell_in_range(office_r, office_c, station1.0, station1.1)
                || Railway::cell_in_range(office_r, office_c, station2.0, station2.1);
            if home_covered && office_covered {
                revenue += dist as i64;
            }
        }
        revenue
    }

    fn is_new_station_pair_investment_profitable(
        &self,
        station1: Pos,
        station2: Pos,
        path_len: usize,
    ) -> bool {
        let revenue_per_turn = self.calc_pair_revenue(station1, station2);
        let station_construct = 1;
        let remaining_turns = self.t - self.loop_count - path_len - station_construct;
        let total_expected_revenue = revenue_per_turn * remaining_turns as i64;
        let investment_cost = 2 * STATION_COST as i64 + (path_len as i64 * RAIL_COST as i64);
        total_expected_revenue >= investment_cost
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        t: usize,
    }

    let mut railway = Railway::new(n, m, k, t);

    // People and Office
    //let mut people = Vec::with_capacity(m);
    for _ in 0..m {
        input! {
            is: usize, js: usize,
            it: usize, jt: usize,
        }
        let st_dt = ((is as i64 - it as i64).abs() + (js as i64 - jt as i64).abs()) as usize;
        railway.home_office.push(((is, js), (it, jt), st_dt));
    }

    railway.create_station_candidate();
    if !railway.find_first_investigate(true) {
        println!("# ** First investigate FALSE change least_dist from 20 to 10 **");
        railway.least_dist = 10;
        if !railway.find_first_investigate(true) {
            println!("# ** First investigate FALSE change least_dist from 10 to 7 **");
            railway.least_dist = 7;
            railway.find_first_investigate(true);
            railway.least_dist = 20;
        }
    }

    let mut outputs = Vec::new();

    while railway.loop_count < railway.t {
        outputs.push(format!("# capital={}", railway.capital));
        outputs.push(format!("# revenue={}", railway.calc_revenue()));
        if !railway.path.is_empty() && railway.path.len() >= 3 {
            if (RAIL_COST as i64) < railway.capital {
                let pos_prev = railway.path[0];
                let pos_cur = railway.path[1];
                let pos_next = railway.path[2];
                let track_type = decide_track_type(
                    pos_prev.0, pos_prev.1, pos_cur.0, pos_cur.1, pos_next.0, pos_next.1,
                );

                railway.grid[pos_cur.0][pos_cur.1] = Cell::Track;

                railway.path.remove(0);
                let tmp = format!("{} {} {}", track_type.to_string(), pos_cur.0, pos_cur.1);
                railway.capital -= RAIL_COST as i64;

                outputs.push(tmp);

                if !(railway.path.len() >= 3) {
                    railway.path = Vec::new();
                }
            } else {
                outputs.push("-1".to_string());
            }
        } else if !railway.stations.is_empty() {
            if railway.is_station() {
                railway.stations.remove(0);
                continue;
            }

            if (STATION_COST as i64) < railway.capital {
                let station = railway.stations.remove(0);
                railway.grid[station.0][station.1] = Cell::Station;
                let tmp = format!("{} {} {}", "0".to_string(), station.0, station.1);
                railway.capital -= STATION_COST as i64;

                outputs.push(tmp);

                if railway.stations.is_empty() {
                    railway.stations = Vec::new();
                }
            } else {
                // println!("# Capital short - station");
                outputs.push("-1".to_string());
            }
        } else {
            if railway.extend_network() {
                // println!("# station={} path={}", railway.stations.len(), railway.path.len());
                continue;
            } else {
                break;
                // println!("# Can not found new stations");
                /*
                if railway.find_first_investigate(false) {
                    println!("# Find first investigate again");
                    continue;
                } else {
                    println!("# Find first investigate again FALSE loop={}", railway.loop_count);
                    break;
                }
                */
            }
        }

        railway.capital += railway.calc_revenue();
        railway.loop_count += 1;
    }

    for _ in railway.loop_count..t {
        outputs.push("-1".to_string());
    }

    for out in outputs {
        println!("{}", out);
    }
}

fn decide_track_type(rp: usize, cp: usize, r: usize, c: usize, rn: usize, cn: usize) -> i64 {
    let dr1 = r as i64 - rp as i64;
    let dc1 = c as i64 - cp as i64;
    let dr2 = rn as i64 - r as i64;
    let dc2 = cn as i64 - c as i64;

    let dir_str_1 = dir_to_char(dr1, dc1);
    let dir_str_2 = dir_to_char(dr2, dc2);

    let combo = format!("{}-{}", dir_str_1, dir_str_2);

    // println!("# {}", combo);

    match combo.as_str() {
        "L-L" | "R-R" => 1,
        "U-U" | "D-D" => 2,

        "R-D" | "U-L" => 3,
        "R-U" | "D-L" => 4,
        "D-R" | "L-U" => 5,
        "U-R" | "L-D" => 6,

        _ => 1,
    }
}

fn dir_to_char(dr: i64, dc: i64) -> &'static str {
    match (dr, dc) {
        (0, 1) => "R",
        (0, -1) => "L",
        (1, 0) => "D",
        (-1, 0) => "U",
        _ => "?",
    }
}
