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

type AttackPower = Vec<Vec<i32>>;

#[derive(Clone, Debug)]
struct Weapon {
    initial_durability: i32,
}

#[derive(Clone, Debug)]
struct Chest {
    #[allow(dead_code)]
    initial_hardness: i32,
}

#[derive(Clone, Debug)]
struct AttackMatrix {
    matrix: AttackPower,
}

impl AttackMatrix {
    fn new(matrix: AttackPower) -> Self {
        Self { matrix }
    }

    fn damage(&self, weapon_id: usize, chest_id: usize) -> i32 {
        self.matrix[weapon_id][chest_id]
    }

    fn weapon_row(&self, weapon_id: usize) -> &[i32] {
        &self.matrix[weapon_id]
    }
}

#[derive(Clone, Debug)]
struct Inventory {
    remaining: Vec<i32>,
}

impl Inventory {
    fn new(weapons: &[Weapon]) -> Self {
        let remaining = weapons.iter().map(|w| w.initial_durability).collect();
        Self { remaining }
    }

    fn remaining(&self, weapon_id: usize) -> i32 {
        self.remaining[weapon_id]
    }

    fn use_once(&mut self, weapon_id: usize) -> bool {
        if self.remaining[weapon_id] <= 0 {
            return false;
        }
        self.remaining[weapon_id] -= 1;
        true
    }
}

#[derive(Clone, Debug)]
struct BattleState {
    chests: Vec<Chest>,
    weapons: Vec<Weapon>,
    attacks: AttackMatrix,
    inventory: Inventory,
    remaining: Vec<i32>,
    opened: Vec<bool>,
    actions: Vec<(i32, usize)>,
    attack_heap: BinaryHeap<AttackCandidate>,
}

impl BattleState {
    fn new(h: Vec<i32>, c: Vec<i32>, a: AttackPower) -> Self {
        let chests = h
            .iter()
            .map(|&hardness| Chest {
                initial_hardness: hardness,
            })
            .collect();
        let weapons = c
            .iter()
            .map(|&durability| Weapon {
                initial_durability: durability,
            })
            .collect::<Vec<_>>();
        let attacks = AttackMatrix::new(a);
        let inventory = Inventory::new(&weapons);
        let remaining = h;
        let opened = vec![false; weapons.len()];
        let actions = Vec::new();
        let attack_heap = BinaryHeap::new();
        Self {
            chests,
            weapons,
            attacks,
            inventory,
            remaining,
            opened,
            actions,
            attack_heap,
        }
    }

    fn len(&self) -> usize {
        self.chests.len()
    }

    fn remaining_hardness(&self, chest_id: usize) -> i32 {
        self.remaining[chest_id]
    }

    fn record_attack(&mut self, weapon: Option<usize>, target: usize, damage: i32) {
        if damage <= 0 {
            return;
        }
        if let Some(w) = weapon {
            self.actions.push((w as i32, target));
        } else {
            self.actions.push((-1, target));
        }
    }

    fn execute_fist(&mut self, target: usize) {
        if self.remaining[target] <= 0 {
            return;
        }
        self.remaining[target] -= 1;
        self.record_attack(None, target, 1);
        if self.remaining[target] <= 0 {
            self.on_chest_opened(target);
        }
    }

    fn execute_weapon_attack(&mut self, weapon_id: usize, target: usize, damage: i32) -> bool {
        if self.remaining[target] <= 0 {
            return false;
        }
        if !self.opened[weapon_id] {
            return false;
        }
        if self.inventory.remaining(weapon_id) <= 0 {
            return false;
        }
        if damage <= 0 {
            return false;
        }
        if !self.inventory.use_once(weapon_id) {
            return false;
        }
        self.remaining[target] = (self.remaining[target] - damage).max(0);
        self.record_attack(Some(weapon_id), target, damage);
        if self.remaining[target] <= 0 {
            self.on_chest_opened(target);
        }
        true
    }

    fn on_chest_opened(&mut self, chest_id: usize) {
        if self.opened[chest_id] {
            return;
        }
        self.opened[chest_id] = true;
        self.enqueue_weapon_attacks(chest_id);
    }

    fn enqueue_weapon_attacks(&mut self, weapon_id: usize) {
        if !self.opened[weapon_id] {
            return;
        }
        if self.inventory.remaining(weapon_id) <= 0 {
            return;
        }
        let remaining = self.inventory.remaining(weapon_id) as usize;
        if remaining == 0 {
            return;
        }
        let weapon_row = self.attacks.weapon_row(weapon_id);
        for chest_id in 0..self.len() {
            if self.remaining[chest_id] <= 0 {
                continue;
            }
            let damage = weapon_row[chest_id];
            if damage <= 1 {
                continue;
            }
            self.attack_heap.push(AttackCandidate {
                damage,
                weapon: weapon_id,
                target: chest_id,
            });
        }
    }

    fn apply_best_attacks(&mut self) {
        while let Some(candidate) = self.attack_heap.pop() {
            if self.inventory.remaining(candidate.weapon) <= 0 {
                continue;
            }
            if !self.opened[candidate.weapon] {
                continue;
            }
            if self.remaining[candidate.target] <= 0 {
                continue;
            }
            let damage = self.attacks.damage(candidate.weapon, candidate.target);
            if damage <= 1 {
                break;
            }
            let success = self.execute_weapon_attack(candidate.weapon, candidate.target, damage);
            if !success {
                continue;
            }
            if self.inventory.remaining(candidate.weapon) > 0
                && self.remaining[candidate.target] > 0
            {
                self.attack_heap.push(candidate);
            }
        }
    }

    fn open_with_fists(&mut self, chest_id: usize) {
        while self.remaining[chest_id] > 0 {
            self.execute_fist(chest_id);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttackCandidate {
    damage: i32,
    weapon: usize,
    target: usize,
}

impl Ord for AttackCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.damage
            .cmp(&other.damage)
            .then_with(|| other.weapon.cmp(&self.weapon))
            .then_with(|| other.target.cmp(&self.target))
    }
}

impl PartialOrd for AttackCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn evaluate_chest(state: &BattleState, chest_id: usize) -> f64 {
    if state.remaining_hardness(chest_id) <= 0 {
        return f64::NEG_INFINITY;
    }
    let weapon = &state.weapons[chest_id];
    let durability = weapon.initial_durability.max(0) as usize;
    if durability == 0 {
        return 0.0;
    }

    let mut contributions = Vec::with_capacity(state.len());
    let weapon_row = state.attacks.weapon_row(chest_id);
    for (target, &hardness) in state.remaining.iter().enumerate() {
        if target == chest_id || hardness <= 0 {
            continue;
        }
        let damage = weapon_row[target];
        if damage <= 0 {
            continue;
        }
        let effective = damage.min(hardness);
        contributions.push(effective);
    }
    if contributions.is_empty() {
        return 0.0;
    }
    contributions.sort_unstable_by(|a, b| b.cmp(a));
    let potential: i32 = contributions.into_iter().take(durability).sum();
    if potential <= 0 {
        return 0.0;
    }
    let cost = state.remaining_hardness(chest_id).max(1) as f64;
    let max_damage = state
        .attacks
        .weapon_row(chest_id)
        .iter()
        .copied()
        .max()
        .unwrap_or(0);
    potential as f64 / cost + (max_damage as f64) * 1e-4
}

fn choose_next_chest(state: &BattleState) -> Option<usize> {
    let mut best_idx = None;
    let mut best_value = f64::NEG_INFINITY;
    for chest_id in 0..state.len() {
        if state.remaining_hardness(chest_id) <= 0 {
            continue;
        }
        let value = evaluate_chest(state, chest_id);
        if value > best_value {
            best_value = value;
            best_idx = Some(chest_id);
        }
    }
    if best_value <= 0.0 {
        state
            .remaining
            .iter()
            .enumerate()
            .filter(|(_, &h)| h > 0)
            .min_by_key(|&(_, h)| h)
            .map(|(idx, _)| idx)
    } else {
        best_idx
    }
}

fn greedy_strategy(state: &mut BattleState) {
    for chest_id in 0..state.len() {
        if state.remaining[chest_id] <= 0 {
            state.on_chest_opened(chest_id);
        }
    }
    while state.remaining.iter().any(|&h| h > 0) {
        state.apply_best_attacks();
        if !state.remaining.iter().any(|&h| h > 0) {
            break;
        }
        let chest_id = choose_next_chest(state).expect("there must be unopened chest");
        state.open_with_fists(chest_id);
        state.apply_best_attacks();
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i32; n],
        c: [i32; n],
        a: [[i32; n]; n],
    };

    let mut state = BattleState::new(h, c, a);
    greedy_strategy(&mut state);

    eprintln!("{}", state.actions.len());
    for (weapon, target) in &state.actions {
        println!("{} {}", weapon, target);
    }
}
