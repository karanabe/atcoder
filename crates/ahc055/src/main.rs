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
    // size(&self, a: usize) -> usize
    // groups(&self) -> Vec<Vec<usize>>
    FenwickTree,
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    Max,
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&self, from: usize, to: usize)
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
    #[allow(dead_code)]
    id: usize,
    initial_durability: i32,
}

#[derive(Clone, Debug)]
struct Chest {
    #[allow(dead_code)]
    id: usize,
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

struct BattleState {
    chests: Vec<Chest>,
    weapons: Vec<Weapon>,
    attacks: AttackMatrix,
    inventory: Inventory,
    opened: Vec<bool>,
    remaining: Vec<i32>,
    actions: Vec<(i32, usize)>,
}

impl BattleState {
    fn new(h: Vec<i32>, c: Vec<i32>, a: AttackPower) -> Self {
        let chests: Vec<Chest> = h
            .iter()
            .enumerate()
            .map(|(id, &initial_hardness)| Chest {
                id,
                initial_hardness,
            })
            .collect();
        let weapons: Vec<Weapon> = c
            .iter()
            .enumerate()
            .map(|(id, &durability)| Weapon {
                id,
                initial_durability: durability,
            })
            .collect();
        let opened = vec![false; weapons.len()];
        let inventory = Inventory::new(&weapons);
        let attacks = AttackMatrix::new(a);
        let remaining = h;
        let actions = Vec::new();
        Self {
            chests,
            weapons,
            attacks,
            inventory,
            opened,
            remaining,
            actions,
        }
    }
}

trait BattleOps {
    fn attacks(&self) -> &AttackMatrix;
    fn chests(&self) -> &[Chest];
    fn weapons(&self) -> &[Weapon];
    fn opened(&self) -> &[bool];
    fn opened_mut(&mut self) -> &mut [bool];
    fn inventory(&self) -> &Inventory;
    fn inventory_mut(&mut self) -> &mut Inventory;
    fn remaining(&self) -> &[i32];
    fn remaining_mut(&mut self) -> &mut [i32];
    fn actions(&self) -> &[(i32, usize)];
    fn actions_mut(&mut self) -> &mut Vec<(i32, usize)>;

    fn is_opened(&self, chest_id: usize) -> bool {
        self.opened()[chest_id]
    }

    fn mark_opened(&mut self, chest_id: usize) {
        self.opened_mut()[chest_id] = true;
    }

    fn record_attack(&mut self, weapon: Option<usize>, target: usize) {
        let weapon_id = weapon.map(|w| w as i32).unwrap_or(-1);
        self.actions_mut().push((weapon_id, target));
    }

    fn best_assist_damage(&self, target: usize) -> i32 {
        let mut best = 0;
        for weapon_id in 0..self.weapons().len() {
            if !self.can_use_weapon(weapon_id) {
                continue;
            }
            let damage = self.attacks().damage(weapon_id, target);
            if damage <= 0 {
                continue;
            }
            let effective = damage.min(self.remaining()[target]);
            if effective > best {
                best = effective;
            }
        }
        best
    }

    fn assist_opening(&mut self, target: usize) {
        loop {
            let mut best_weapon = None;
            let mut best_effective = 0;
            for weapon_id in 0..self.weapons().len() {
                if !self.can_use_weapon(weapon_id) {
                    continue;
                }
                let damage = self.attacks().damage(weapon_id, target);
                if damage <= 1 {
                    continue;
                }
                let effective = damage.min(self.remaining()[target]);
                if effective > best_effective {
                    best_effective = effective;
                    best_weapon = Some(weapon_id);
                }
            }

            let Some(weapon_id) = best_weapon else {
                break;
            };

            if !self.execute_attack(Some(weapon_id), target) {
                break;
            }

            if self.remaining()[target] <= 0 {
                break;
            }
        }
    }

    fn can_use_weapon(&self, weapon_id: usize) -> bool {
        self.is_opened(weapon_id) && self.inventory().remaining(weapon_id) > 0
    }

    fn evaluate_unlock(&self, chest_id: usize) -> f64 {
        if self.remaining()[chest_id] <= 0 {
            return f64::NEG_INFINITY;
        }
        let weapon = &self.weapons()[chest_id];

        let mut gains: Vec<i32> = self
            .attacks()
            .weapon_row(chest_id)
            .iter()
            .enumerate()
            .filter(|(target, _)| *target != chest_id && self.remaining()[*target] > 0)
            .map(|(_, &damage)| damage)
            .collect();
        gains.sort_unstable_by(|a, b| b.cmp(a));
        let total = gains
            .into_iter()
            .take(weapon.initial_durability as usize)
            .sum::<i32>();

        let assist = self.best_assist_damage(chest_id);
        let adjusted_cost = (self.remaining()[chest_id] - assist).max(1);

        total as f64 / adjusted_cost as f64
    }

    fn best_target_for_weapon(&self, weapon_id: usize) -> Option<usize> {
        let mut best_target = None;
        let mut best_effective_damage = 0;
        for (target, &remaining) in self.remaining().iter().enumerate() {
            if remaining <= 0 {
                continue;
            }
            let damage = self.attacks().damage(weapon_id, target);
            let effective = damage.min(remaining);
            if effective > best_effective_damage {
                best_effective_damage = effective;
                best_target = Some(target);
            }
        }
        best_target
    }

    fn execute_attack(&mut self, weapon: Option<usize>, target: usize) -> bool {
        if self.remaining()[target] <= 0 {
            return false;
        }

        let damage = match weapon {
            None => 1,
            Some(w) => {
                if !self.can_use_weapon(w) {
                    return false;
                }
                let damage = self.attacks().damage(w, target);
                if damage <= 0 {
                    return false;
                }
                if !self.inventory_mut().use_once(w) {
                    return false;
                }
                damage
            }
        };

        {
            let remaining = self.remaining_mut();
            remaining[target] = (remaining[target] - damage).max(0);
        }
        self.record_attack(weapon, target);

        if self.remaining()[target] <= 0 && !self.is_opened(target) {
            self.mark_opened(target);
        }
        true
    }

    fn use_weapon_greedily(&mut self, weapon_id: usize) {
        while self.can_use_weapon(weapon_id) {
            let Some(target) = self.best_target_for_weapon(weapon_id) else {
                break;
            };
            if !self.execute_attack(Some(weapon_id), target) {
                break;
            }
        }
    }
}

impl BattleOps for BattleState {
    fn attacks(&self) -> &AttackMatrix {
        &self.attacks
    }
    fn chests(&self) -> &[Chest] {
        &self.chests
    }
    fn weapons(&self) -> &[Weapon] {
        &self.weapons
    }
    fn opened(&self) -> &[bool] {
        &self.opened
    }
    fn opened_mut(&mut self) -> &mut [bool] {
        &mut self.opened
    }
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }
    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
    fn remaining(&self) -> &[i32] {
        &self.remaining
    }
    fn remaining_mut(&mut self) -> &mut [i32] {
        &mut self.remaining
    }
    fn actions(&self) -> &[(i32, usize)] {
        &self.actions
    }
    fn actions_mut(&mut self) -> &mut Vec<(i32, usize)> {
        &mut self.actions
    }
}

fn greedy_solve(state: &mut BattleState) {
    let n = state.chests().len();
    while state.remaining().iter().any(|&hardness| hardness > 0) {
        let mut best_idx = None;
        let mut best_value = f64::NEG_INFINITY;
        for chest_id in 0..n {
            if state.remaining()[chest_id] <= 0 {
                continue;
            }
            let value = state.evaluate_unlock(chest_id);
            if value > best_value {
                best_value = value;
                best_idx = Some(chest_id);
            }
        }

        let chest_id = best_idx.unwrap_or_else(|| {
            state
                .remaining()
                .iter()
                .enumerate()
                .find(|(_, &hardness)| hardness > 0)
                .map(|(idx, _)| idx)
                .expect("there must be a closed chest")
        });

        state.assist_opening(chest_id);

        while state.remaining()[chest_id] > 0 {
            if !state.execute_attack(None, chest_id) {
                break;
            }
        }

        if !state.is_opened(chest_id) {
            state.mark_opened(chest_id);
        }
        state.use_weapon_greedily(chest_id);
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
    greedy_solve(&mut state);

    eprintln!("{}", state.actions().len());
    for (weapon, target) in state.actions() {
        println!("{} {}", weapon, target);
    }
}
