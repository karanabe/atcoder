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
use std::io::{self, BufReader, StdinLock, Write};

#[allow(unused_imports)]
use rand::{prelude::*, rngs::StdRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};
#[allow(unused_imports)]
use rand_distr::{Distribution, Normal};
#[allow(unused_imports)]
use std::time::{Duration, Instant};

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let r = self.find(self.par[x]);
            self.par[x] = r;
            r
        }
    }
    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            false
        } else {
            if self.rank[rx] < self.rank[ry] {
                self.par[rx] = ry;
            } else {
                self.par[ry] = rx;
                if self.rank[rx] == self.rank[ry] {
                    self.rank[rx] += 1;
                }
            }
            true
        }
    }
}

struct DistModel {
    rank_sum_map: HashMap<(usize, usize), (f64, f64)>,
    coords: Vec<(f64, f64)>,
}
impl DistModel {
    fn new(coords: Vec<(f64, f64)>) -> Self {
        Self {
            rank_sum_map: HashMap::new(),
            coords,
        }
    }

    fn add_rank_info(&mut self, edges: &[(usize, usize, usize)]) {
        for &(u, v, r) in edges {
            let (a, b) = if u < v { (u, v) } else { (v, u) };
            let entry = self.rank_sum_map.entry((a, b)).or_insert((0.0, 0.0));
            entry.0 += r as f64;
            entry.1 += 1.0;
        }
    }

    fn dist(&self, u: usize, v: usize) -> f64 {
        let dx = self.coords[u].0 - self.coords[v].0;
        let dy = self.coords[u].1 - self.coords[v].1;
        let base = (dx * dx + dy * dy).sqrt();
        let (a, b) = if u < v { (u, v) } else { (v, u) };
        if let Some(&(rank_sum, count)) = self.rank_sum_map.get(&(a, b)) {
            let avg_rank = rank_sum / count;

            let factor = 0.95_f64.powf(avg_rank - 1.0);
            base * factor
        } else {
            base
        }
    }
}

fn group_mst_cost(group: &[usize], dist_model: &DistModel) -> f64 {
    if group.len() <= 1 {
        return 0.0;
    }
    let n = dist_model.coords.len();
    let mut edges = Vec::with_capacity(group.len() * (group.len() - 1) / 2);
    for i in 0..group.len() {
        for j in i + 1..group.len() {
            let u = group[i];
            let v = group[j];
            let d = dist_model.dist(u, v);
            edges.push((d, u, v));
        }
    }
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    let mut uf = UnionFind::new(n);
    let mut cost = 0.0;
    let mut used = 0;
    for &(d, u, v) in &edges {
        if uf.union(u, v) {
            cost += d;
            used += 1;
            if used + 1 == group.len() {
                break;
            }
        }
    }
    cost
}

fn compute_total_cost(_groups: &[Vec<usize>], groups_cost: &[f64]) -> f64 {
    groups_cost.iter().sum()
}
fn recalc_cost_for_groups(
    groups: &mut [Vec<usize>],
    groups_cost: &mut [f64],
    dist_model: &DistModel,
    changed_ids: &[usize],
    old_total: f64,
) -> f64 {
    let mut new_total = old_total;
    for &gid in changed_ids {
        let old_val = groups_cost[gid];
        let new_val = group_mst_cost(&groups[gid], dist_model);
        groups_cost[gid] = new_val;
        new_total += new_val - old_val;
    }
    new_total
}

fn try_local_move(
    groups: &mut [Vec<usize>],
    groups_cost: &mut [f64],
    dist_model: &DistModel,
    total_cost: f64,
    op_type: usize,
    rng: &mut impl Rng,
) -> (f64, Vec<usize>, Vec<(usize, usize, usize)>) {
    let mut record = Vec::new();
    let n_gr = groups.len();
    let old_total = total_cost;

    match op_type {
        0 => {
            if n_gr < 2 {
                return (old_total, vec![], record);
            }
            let g1 = rng.gen_range(0..n_gr);
            let mut g2 = rng.gen_range(0..n_gr);
            if g1 == g2 && n_gr > 1 {
                g2 = (g2 + 1) % n_gr;
            }
            if groups[g1].is_empty() || groups[g2].is_empty() {
                return (old_total, vec![], record);
            }
            let i1 = rng.gen_range(0..groups[g1].len());
            let i2 = rng.gen_range(0..groups[g2].len());
            let c1 = groups[g1][i1];
            let c2 = groups[g2][i2];

            record.push((g1, i1, c1));
            record.push((g2, i2, c2));

            groups[g1][i1] = c2;
            groups[g2][i2] = c1;

            let changed = vec![g1, g2];
            let new_total =
                recalc_cost_for_groups(groups, groups_cost, dist_model, &changed, old_total);
            (new_total, changed, record)
        }
        1 => {
            if n_gr < 3 {
                return (old_total, vec![], record);
            }
            let mut arr = (0..n_gr).collect_vec();
            arr.shuffle(rng);
            let g1 = arr[0];
            let g2 = arr[1];
            let g3 = arr[2];
            if groups[g1].is_empty() || groups[g2].is_empty() || groups[g3].is_empty() {
                return (old_total, vec![], record);
            }
            let i1 = rng.gen_range(0..groups[g1].len());
            let i2 = rng.gen_range(0..groups[g2].len());
            let i3 = rng.gen_range(0..groups[g3].len());
            let c1 = groups[g1][i1];
            let c2 = groups[g2][i2];
            let c3 = groups[g3][i3];

            record.push((g1, i1, c1));
            record.push((g2, i2, c2));
            record.push((g3, i3, c3));

            groups[g1][i1] = c3;
            groups[g2][i2] = c1;
            groups[g3][i3] = c2;
            let changed = vec![g1, g2, g3];
            let new_total =
                recalc_cost_for_groups(groups, groups_cost, dist_model, &changed, old_total);
            (new_total, changed, record)
        }
        2 => {
            if n_gr < 2 {
                return (old_total, vec![], record);
            }
            let g1 = rng.gen_range(0..n_gr);
            if groups[g1].is_empty() {
                return (old_total, vec![], record);
            }
            let mut g2 = rng.gen_range(0..n_gr);
            if g1 == g2 && n_gr > 1 {
                g2 = (g2 + 1) % n_gr;
            }
            let i1 = rng.gen_range(0..groups[g1].len());
            let c1 = groups[g1][i1];

            record.push((g1, i1, c1));

            groups[g1].swap_remove(i1);
            let inserted_pos = groups[g2].len();
            groups[g2].push(c1);
            record.push((g2, inserted_pos, c1));
            let changed = vec![g1, g2];
            let new_total =
                recalc_cost_for_groups(groups, groups_cost, dist_model, &changed, old_total);
            (new_total, changed, record)
        }
        _ => (old_total, vec![], record),
    }
}

fn rollback_local_move(
    groups: &mut [Vec<usize>],
    groups_cost: &mut [f64],
    dist_model: &DistModel,
    changed_ids: &[usize],
    _old_total: f64,
    record: &[(usize, usize, usize)],
) {
    for &(gid, pos, city_before) in record {
        if pos < groups[gid].len() {
            groups[gid][pos] = city_before;
        } else {
            groups[gid].push(city_before);
        }
    }

    for &gid in changed_ids {
        groups_cost[gid] = group_mst_cost(&groups[gid], dist_model);
    }
}

fn simulated_annealing(groups: &mut Vec<Vec<usize>>, dist_model: &DistModel, time_limit_sec: f64) {
    let start = Instant::now();
    let end_t = start + Duration::from_secs_f64(time_limit_sec);
    let mut rng = thread_rng();

    let mut groups_cost = vec![0.0; groups.len()];
    for (gid, g) in groups.iter().enumerate() {
        groups_cost[gid] = group_mst_cost(g, dist_model);
    }
    let mut total_cost = compute_total_cost(groups, &groups_cost);

    let mut best_groups = groups.to_vec();
    let mut best_groups_cost = groups_cost.clone();
    let mut best_score = total_cost;

    let t0: f64 = 1e-1;
    let t1: f64 = 1e-4;

    let mut iter_count = 0;
    while Instant::now() < end_t {
        iter_count += 1;
        let now = Instant::now();
        let progress = (now - start).as_secs_f64() / time_limit_sec;
        if progress >= 1.0 {
            break;
        }
        let temp = t0.powf(1.0 - progress) * t1.powf(progress);

        let op_type = rng.gen_range(0..1);

        let (new_cost, changed, record) = try_local_move(
            groups,
            &mut groups_cost,
            dist_model,
            total_cost,
            op_type,
            &mut rng,
        );
        let delta = new_cost - total_cost;

        if delta < 0.0 {
            total_cost = new_cost;
            if new_cost < best_score {
                best_score = new_cost;
                best_groups = groups.to_vec();
                best_groups_cost.copy_from_slice(&groups_cost);
            }
        } else {
            let p = f64::exp(-delta / temp);
            if rng.gen::<f64>() >= p {
                rollback_local_move(
                    groups,
                    &mut groups_cost,
                    dist_model,
                    &changed,
                    total_cost,
                    &record,
                );
            } else {
                total_cost = new_cost;
            }
        }
    }

    eprintln!(
        "SA finished. iter={}, best_cost={:.2}",
        iter_count, best_score
    );

    *groups = best_groups;
    groups_cost.copy_from_slice(&best_groups_cost);
}

fn main() {
    let stdin = io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        m: usize,
        q_max: usize,
        l: usize,
        _w: usize,
    }
    input! {
        from &mut source,
        group_sizes: [usize; m],
    }

    let mut boxes = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            from &mut source,
            lx: f64,
            rx: f64,
            ly: f64,
            ry: f64,
        }

        let cx = (lx + rx) * 0.5;
        let cy = (ly + ry) * 0.5;
        boxes.push((i, cx, cy));
    }

    let mut coords = vec![(0.0, 0.0); n];
    for &(id, x, y) in &boxes {
        coords[id] = (x, y);
    }

    let mut dist_model = DistModel::new(coords.clone());

    let clusters = bfs_clustering(n, &coords, 2000.0);

    let mut rng = thread_rng();
    let mut cluster_ids: Vec<_> = (0..clusters.len()).collect();
    cluster_ids.shuffle(&mut rng);

    let mut subsets = Vec::new();
    let mut used_q = 0;

    for &cid in &cluster_ids {
        if used_q >= q_max {
            break;
        }
        let c = &clusters[cid];
        if c.len() < 2 {
            continue;
        }

        let lsize = c.len().min(l);
        let mut chosen = HashSet::new();
        while chosen.len() < lsize {
            let x = c[rng.gen_range(0..c.len())];
            chosen.insert(x);
        }
        subsets.push(chosen.into_iter().collect_vec());
        used_q += 1;
    }

    boxes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    let step = (n / 20).max(1);
    for start in (0..n).step_by(step) {
        if used_q >= q_max {
            break;
        }
        if start + l > n {
            break;
        }
        let sub = boxes[start..start + l]
            .iter()
            .map(|&(id, _, _)| id)
            .collect_vec();
        subsets.push(sub);
        used_q += 1;
    }

    while used_q < q_max {
        let mut set = HashSet::new();
        while set.len() < l {
            set.insert(rng.gen_range(0..n));
        }
        subsets.push(set.into_iter().collect_vec());
        used_q += 1;
    }

    for sub in &subsets {
        if sub.len() < 2 {
            continue;
        }
        print!("? {}", sub.len());
        for &c in sub {
            print!(" {}", c);
        }
        println!();
        let _ = io::stdout().flush();

        let lsize = sub.len();
        let mut mst_edges = Vec::with_capacity(lsize - 1);
        for i in 0..(lsize - 1) {
            input! {
                from &mut source,
                a: usize,
                b: usize,
            }
            let (x, y) = if a < b { (a, b) } else { (b, a) };
            mst_edges.push((x, y, i + 1));
        }
        dist_model.add_rank_info(&mst_edges);

        let not_adopted = build_not_adopted_edges(sub, &mst_edges, lsize - 1);
        dist_model.add_rank_info(&not_adopted);
    }

    println!("!");
    let _ = io::stdout().flush();

    let mut groups = build_initial_groups_by_clusters(n, &clusters, &group_sizes);

    simulated_annealing(&mut groups, &dist_model, 1.88);

    for g in &groups {
        for (i, &city) in g.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", city);
        }
        println!();

        if g.len() <= 1 {
            continue;
        }
        let mut edges = Vec::with_capacity(g.len() * (g.len() - 1) / 2);
        for i in 0..g.len() {
            for j in i + 1..g.len() {
                let u = g[i];
                let v = g[j];
                let d = dist_model.dist(u, v);
                edges.push((d, u, v));
            }
        }
        edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
        let mut uf = UnionFind::new(n);
        let mut used = 0;
        for &(_, u, v) in &edges {
            if uf.union(u, v) {
                let (a, b) = if u < v { (u, v) } else { (v, u) };
                println!("{} {}", a, b);
                used += 1;
                if used + 1 == g.len() {
                    break;
                }
            }
        }
    }
}

fn bfs_clustering(n: usize, coords: &[(f64, f64)], threshold: f64) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];

    let mut idxs: Vec<usize> = (0..n).collect();
    idxs.sort_by(|&a, &b| {
        coords[a]
            .0
            .partial_cmp(&coords[b].0)
            .unwrap_or(Ordering::Equal)
    });
    let check_range = 120;
    for i in 0..n {
        let ci = idxs[i];
        for j in i + 1..(i + check_range).min(n) {
            let cj = idxs[j];
            let dx = coords[ci].0 - coords[cj].0;
            if dx * dx > threshold * threshold {
                break;
            }
            let dy = coords[ci].1 - coords[cj].1;
            if dx * dx + dy * dy <= threshold * threshold {
                adj[ci].push(cj);
                adj[cj].push(ci);
            }
        }
    }

    let mut visited = vec![false; n];
    let mut clusters = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let mut comp = vec![];
            visited[i] = true;
            let mut que = VecDeque::new();
            que.push_back(i);
            while let Some(u) = que.pop_front() {
                comp.push(u);
                for &nx in &adj[u] {
                    if !visited[nx] {
                        visited[nx] = true;
                        que.push_back(nx);
                    }
                }
            }
            clusters.push(comp);
        }
    }
    clusters
}

fn build_not_adopted_edges(
    sub: &[usize],
    adopted: &[(usize, usize, usize)],
    large_rank: usize,
) -> Vec<(usize, usize, usize)> {
    let mut set_adopted = HashSet::new();
    for &(u, v, _) in adopted {
        set_adopted.insert((u, v));
    }
    let mut res = Vec::new();
    for i in 0..sub.len() {
        for j in i + 1..sub.len() {
            let (a, b) = {
                let x = sub[i];
                let y = sub[j];
                if x < y {
                    (x, y)
                } else {
                    (y, x)
                }
            };
            if !set_adopted.contains(&(a, b)) {
                res.push((a, b, large_rank));
            }
        }
    }
    res
}

fn build_initial_groups_by_clusters(
    n: usize,
    clusters: &Vec<Vec<usize>>,
    group_sizes: &[usize],
) -> Vec<Vec<usize>> {
    let all_city = clusters.iter().flatten().copied().collect_vec();
    if all_city.len() < n {}

    let mut groups = Vec::new();
    let mut idx = 0;
    for &sz in group_sizes {
        let mut g = Vec::with_capacity(sz);
        for _ in 0..sz {
            if idx < all_city.len() {
                g.push(all_city[idx]);
                idx += 1;
            } else {
                break;
            }
        }
        groups.push(g);
    }
    groups
}
