#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <crate-name>" >&2
  exit 1
fi

crate="$1"
crate_dir="crates/${crate}"

if [[ -e "${crate_dir}" ]]; then
  echo "Error: ${crate_dir} already exists." >&2
  exit 1
fi

cargo new --bin --vcs none "${crate_dir}"

cat > "${crate_dir}/Cargo.toml" <<EOF
[package]
name = "${crate}"
version = "0.1.0"
edition = "2024"
rust-version = "1.89"

[dependencies]
ac-library-rs = "0.2.0"
proconio = "0.5.0"
itertools = "0.14.0"
regex = "1.11.2"
num = "0.4.3"
rustc-hash = "2.1.1"
EOF

rm -f "${crate_dir}/src/main.rs"
mkdir -p "${crate_dir}/src/bin"

write_template() {
  cat <<'EOF'
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{Ordering, max, min};

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
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    math,
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

fn main() {}
EOF
}

for problem in a b c d e f; do
  write_template > "${crate_dir}/src/bin/${problem}.rs"
done
