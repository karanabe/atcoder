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


fn main() {
    input! {
        n: usize,
        board: [Chars; n],
    }
    let mut ops = Vec::new();

    // 盤面上の各駒について探索
    // ここでは各鬼 (i, j) について安全な方向と必要な回数を調べる
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == 'x' {
                let mut best_dir = None;
                let mut best_cost = n+1;  // 十分大きな初期値

                // 上方向をチェック：上側に 'o' がなければ安全
                if (0..i).all(|k| board[k][j] != 'o') {
                    let cost = i + 1;
                    if cost < best_cost {
                        best_cost = cost;
                        best_dir = Some(('U', 'D'));  // Uで取り出し，Dで元に戻す
                    }
                }
                // 下方向をチェック
                if ((i+1)..n).all(|k| board[k][j] != 'o') {
                    let cost = n - i;
                    if cost < best_cost {
                        best_cost = cost;
                        best_dir = Some(('D', 'U'));
                    }
                }
                // 左方向をチェック
                if (0..j).all(|k| board[i][k] != 'o') {
                    let cost = j + 1;
                    if cost < best_cost {
                        best_cost = cost;
                        best_dir = Some(('L', 'R'));
                    }
                }
                // 右方向をチェック
                if ((j+1)..n).all(|k| board[i][k] != 'o') {
                    let cost = n - j;
                    if cost < best_cost {
                        best_cost = cost;
                        best_dir = Some(('R', 'L'));
                    }
                }

                // 安全な方向が見つかった場合、その操作列を追加
                if let Some((d, rev)) = best_dir {
                    // d を best_cost 回、rev を best_cost 回追加
                    for _ in 0..best_cost {
                        // 操作は対象となる行または列の番号も必要
                        // ここでは、d, rev が列操作の場合は j, 行操作の場合は i を出力
                        // たとえば、U, D の場合は「j」を使う
                        match d {
                            'U' | 'D' => ops.push((d, j)),
                            'L' | 'R' => ops.push((d, i)),
                            _ => unreachable!(),
                        }
                    }
                    for _ in 0..best_cost {
                        match rev {
                            'U' | 'D' => ops.push((rev, j)),
                            'L' | 'R' => ops.push((rev, i)),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }

    // ※ ここで、同じ行／列に対する操作のまとめ（グループ化）を行えばさらに手数が減る
    // println!("{}", ops.len());
    for (d, p) in ops {
        println!("{} {}", d, p);
    }
}
