#[allow(unused_imports)]
use proconio::{
    input,
    fastout,
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
    max
};

fn num_cells_on_main_diagonal(k: i64, n: i64) -> i64 {
    if 2 <= k && k <= n + 1 {
        k - 1
    } else if n + 2 <= k && k <= 2 * n {
        2 * n - k + 1
    } else {
        0
    }
}

fn num_cells_on_sub_diagonal(k: i64, n: i64) -> i64 {
    if -n + 1 <= k && k <= n - 1 {
        n - k.abs()
    } else {
        0
    }
}

fn main() {
    input! {
        n: i64,
        m: usize,
        ab: [(i64, i64); m],
    }

    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut pick_main_diag = HashSet::new();
    let mut pick_sub_diag = HashSet::new();

    for &(a, b) in &ab {
        rows.insert(a);
        cols.insert(b);
        pick_main_diag.insert(a + b);
        pick_sub_diag.insert(a - b);
    }

    let pick_rows = rows.len() as i64;
    let pick_cols = cols.len() as i64;

    let total_cells = n * n;

    let mut ttl_pick = 0i64;

    ttl_pick += n * pick_rows;

    ttl_pick += n * pick_cols;

    // pick main diag
    let mut pick_m_diag_cells = 0i64;
    for &k in &pick_main_diag {
        pick_m_diag_cells += num_cells_on_main_diagonal(k, n);
    }
    ttl_pick += pick_m_diag_cells;

    // pick sub diag
    let mut pick_a_diag_cells = 0i64;
    for &k in &pick_sub_diag {
        pick_a_diag_cells += num_cells_on_sub_diagonal(k, n);
    }
    ttl_pick += pick_a_diag_cells;

    let overlaps_row_col = pick_rows * pick_cols;
    ttl_pick -= overlaps_row_col;

    // overlaps row and main diag
    let mut overlaps_row_main_diag = 0i64;
    for &i in &rows {
        for &k in &pick_main_diag {
            let j = k - i;
            if j >= 1 && j <= n {
                overlaps_row_main_diag += 1;
            }
        }
    }
    ttl_pick -= overlaps_row_main_diag;

    // overlaps row and sub diag
    let mut overlaps_row_sub_diag = 0i64;
    for &i in &rows {
        for &k in &pick_sub_diag {
            let j = i - k;
            if j >= 1 && j <= n {
                overlaps_row_sub_diag += 1;
            }
        }
    }
    ttl_pick -= overlaps_row_sub_diag;

    // overlaps col and main diag
    let mut overlaps_col_main_diag = 0i64;
    for &j in &cols {
        for &k in &pick_main_diag {
            let i = k - j;
            if i >= 1 && i <= n {
                overlaps_col_main_diag += 1;
            }
        }
    }
    ttl_pick -= overlaps_col_main_diag;

    // overlaps col and sub diag
    let mut overlaps_col_sub_diag = 0i64;
    for &j in &cols {
        for &k in &pick_sub_diag {
            let i = k + j;
            if i >= 1 && i <= n {
                overlaps_col_sub_diag += 1;
            }
        }
    }
    ttl_pick -= overlaps_col_sub_diag;

    // overlaps main and sub diag
    let mut overlaps_main_diag_sub_diag = 0i64;
    for &k1 in &pick_main_diag {
        for &k2 in &pick_sub_diag {
            if (k1 + k2) % 2 == 0 {
                let i = (k1 + k2) / 2;
                let j = (k1 - k2) / 2;
                if i >= 1 && i <= n && j >= 1 && j <= n {
                    overlaps_main_diag_sub_diag += 1;
                }
            }
        }
    }
    ttl_pick -= overlaps_main_diag_sub_diag;

    let mut overlaps_row_col_main_diag = 0i64;
    for &i in &rows {
        for &j in &cols {
            let k = i + j;
            if pick_main_diag.contains(&k) {
                overlaps_row_col_main_diag += 1;
            }
        }
    }
    ttl_pick += overlaps_row_col_main_diag;

    let mut overlaps_row_col_sub_diag = 0i64;
    for &i in &rows {
        for &j in &cols {
            let k = i - j;
            if pick_sub_diag.contains(&k) {
                overlaps_row_col_sub_diag += 1;
            }
        }
    }
    ttl_pick += overlaps_row_col_sub_diag;

    let mut overlaps_row_main_diag_sub_diag = 0i64;
    for &i in &rows {
        for &k1 in &pick_main_diag {
            for &k2 in &pick_sub_diag {
                if i == (k1 + k2) / 2 && (k1 + k2) % 2 == 0 {
                    let j = (k1 - k2) / 2;
                    if j >= 1 && j <= n {
                        overlaps_row_main_diag_sub_diag += 1;
                    }
                }
            }
        }
    }
    ttl_pick += overlaps_row_main_diag_sub_diag;

    let mut overlaps_col_main_diag_sub_diag = 0i64;
    for &j in &cols {
        for &k1 in &pick_main_diag {
            for &k2 in &pick_sub_diag {
                if j == (k1 - k2) / 2 && (k1 - k2) % 2 == 0 {
                    let i = (k1 + k2) / 2;
                    if i >= 1 && i <= n {
                        overlaps_col_main_diag_sub_diag += 1;
                    }
                }
            }
        }
    }
    ttl_pick += overlaps_col_main_diag_sub_diag;

    // Finally, check all overlaps
    let mut overlaps_all = 0i64;
    for &i in &rows {
        for &j in &cols {
            if pick_main_diag.contains(&(i + j)) && pick_sub_diag.contains(&(i - j)) {
                overlaps_all += 1;
            }
        }
    }
    ttl_pick -= overlaps_all;

    let ans = total_cells - ttl_pick;
    println!("{}", ans);
}
