use proconio::input;
use std::collections::{BTreeSet, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        queries: [(usize, usize); q],
    }

    let mut row_walls = vec![BTreeSet::new(); h + 1];
    let mut col_walls = vec![BTreeSet::new(); w + 1];
    let mut wall_count = h * w;

    for row in row_walls.iter_mut().take(h + 1).skip(1) {
        row.extend(1..=w);
    }
    for col in col_walls.iter_mut().take(w + 1).skip(1) {
        col.extend(1..=h);
    }

    for (r, c) in queries {
        let mut exploded = false;

        if row_walls[r].contains(&c) {
            row_walls[r].remove(&c);
            col_walls[c].remove(&r);
            wall_count -= 1;
            exploded = true;
        }

        if !exploded {
            let mut targets = HashSet::new();

            if let Some(&nr) = col_walls[c].range(..r).next_back() {
                targets.insert((nr, c));
            }

            if let Some(&nr) = col_walls[c].range(r + 1..).next() {
                targets.insert((nr, c));
            }

            if let Some(&nc) = row_walls[r].range(..c).next_back() {
                targets.insert((r, nc));
            }

            if let Some(&nc) = row_walls[r].range(c + 1..).next() {
                targets.insert((r, nc));
            }

            for (tr, tc) in targets {
                if row_walls[tr].remove(&tc) {
                    col_walls[tc].remove(&tr);
                    wall_count -= 1;
                }
            }
        }
    }

    println!("{}", wall_count);
}
