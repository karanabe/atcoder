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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn safe_up(board: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    (0..i).all(|r| board[r][j] != 'o')
}
fn safe_down(board: &Vec<Vec<char>>, i: usize, j: usize, n: usize) -> bool {
    ((i + 1)..n).all(|r| board[r][j] != 'o')
}
fn safe_left(board: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    (0..j).all(|c| board[i][c] != 'o')
}
fn safe_right(board: &Vec<Vec<char>>, i: usize, j: usize, n: usize) -> bool {
    ((j + 1)..n).all(|c| board[i][c] != 'o')
}

fn has_demon(board: &Vec<Vec<char>>) -> bool {
    board.iter().flatten().any(|&c| c == 'x')
}

fn count_x_up(board: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    for r in (0..i).rev() {
        if board[r][j] == 'o' {
            break;
        }
        if board[r][j] == 'x' {
            count += 1;
        }
    }
    count
}

fn count_x_down(board: &Vec<Vec<char>>, i: usize, j: usize, n: usize) -> usize {
    let mut count = 0;
    for r in (i + 1)..n {
        if board[r][j] == 'o' {
            break;
        }
        if board[r][j] == 'x' {
            count += 1;
        }
    }
    count
}

fn count_x_left(board: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    for c in (0..j).rev() {
        if board[i][c] == 'o' {
            break;
        }
        if board[i][c] == 'x' {
            count += 1;
        }
    }
    count
}

fn count_x_right(board: &Vec<Vec<char>>, i: usize, j: usize, n: usize) -> usize {
    let mut count = 0;
    for c in (j + 1)..n {
        if board[i][c] == 'o' {
            break;
        }
        if board[i][c] == 'x' {
            count += 1;
        }
    }
    count
}

fn update_board(t: usize, i: usize, j: usize, direction: Direction, board: &mut Vec<Vec<char>>) {
    let n = board.len();
    for _ in 0..t {
        match direction {
            Direction::Up => {
                for r in 0..(n - 1) {
                    board[r][j] = board[r + 1][j];
                }
                board[n - 1][j] = '.';
            }
            Direction::Down => {
                for r in (1..n).rev() {
                    board[r][j] = board[r - 1][j];
                }
                board[0][j] = '.';
            }
            Direction::Left => {
                for c in 0..(n - 1) {
                    board[i][c] = board[i][c + 1];
                }
                board[i][n - 1] = '.';
            }
            Direction::Right => {
                for c in (1..n).rev() {
                    board[i][c] = board[i][c - 1];
                }
                board[i][0] = '.';
            }
        }
    }
}

type BestScore = (usize, usize, usize, usize, Direction);

fn best_score(best: BestScore, score: usize, i: usize, j: usize, dir: Direction) -> BestScore {
    // println!("best_score={} score={}", best.1, score);
    if score < best.1 {
        match dir {
            Direction::Up => {
                let t = i + 1;
                return (t, score, i, j, dir);
            }
            Direction::Down => {
                let t = 20 - i;
                return (t, score, i, j, dir);
            }
            Direction::Left => {
                let t = j + 1;
                return (t, score, i, j, dir);
            }
            Direction::Right => {
                let t = 20 - j;
                return (t, score, i, j, dir);
            }
        }
    }
    return best;
}

fn check_guarantee(board: &Vec<Vec<char>>, n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == 'x' {
                if !(safe_up(board, i, j)
                    || safe_down(board, i, j, n)
                    || safe_left(board, i, j)
                    || safe_right(board, i, j, n))
                {
                    //println!("i={i} j={j}");
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        board_input: [Chars; n],
    }
    let mut board: Vec<Vec<char>> = board_input.into_iter().map(|row| row).collect();
    let mut ops: Vec<String> = Vec::new();

    while has_demon(&board) {
        let mut best: BestScore = (0, 100, 0, 0, Direction::Up);

        for i in 0..n {
            for j in 0..n {
                if board[i][j] == 'x' {
                    if safe_up(&board, i, j) {
                        let count = count_x_up(&board, i, j) + 1;
                        let score = (i + 1) / count;
                        best = best_score(best, score, i, j, Direction::Up);
                    }

                    if safe_down(&board, i, j, n) {
                        let count = count_x_down(&board, i, j, n) + 1;
                        let score = (20 - i) / count;
                        best = best_score(best, score, i, j, Direction::Down);
                    }

                    if safe_left(&board, i, j) {
                        let count = count_x_left(&board, i, j) + 1;
                        let score = (j + 1) / count;
                        best = best_score(best, score, i, j, Direction::Left);
                    }

                    if safe_right(&board, i, j, n) {
                        let count = count_x_right(&board, i, j, n) + 1;
                        let score = (20 - j) / count;
                        best = best_score(best, score, i, j, Direction::Right);
                    }
                }
            }
        }

        for _ in 0..best.0 {
            if best.4 == Direction::Up {
                let tmp = format!("U {}", best.3);
                ops.push(tmp);
            } else if best.4 == Direction::Down {
                let tmp = format!("D {}", best.3);
                ops.push(tmp);
            } else if best.4 == Direction::Left {
                let tmp = format!("L {}", best.2);
                ops.push(tmp);
            } else if best.4 == Direction::Right {
                let tmp = format!("R {}", best.2);
                ops.push(tmp);
            }
        }

        update_board(best.0, best.2, best.3, best.4, &mut board);

        for _ in 0..best.0 {
            if check_guarantee(&board, n) {
                break;
            }
            if best.4 == Direction::Up {
                let tmp = format!("D {}", best.3);
                ops.push(tmp);
                update_board(1, best.2, best.3, Direction::Down, &mut board);
            } else if best.4 == Direction::Down {
                let tmp = format!("U {}", best.3);
                ops.push(tmp);
                update_board(1, best.2, best.3, Direction::Up, &mut board);
            } else if best.4 == Direction::Left {
                let tmp = format!("R {}", best.2);
                ops.push(tmp);
                update_board(1, best.2, best.3, Direction::Right, &mut board);
            } else if best.4 == Direction::Right {
                let tmp = format!("L {}", best.2);
                ops.push(tmp);
                update_board(1, best.2, best.3, Direction::Left, &mut board);
            }
        }
    }

    // eprintln!("score={:?}", best);

    for op in ops {
        println!("{}", op);
    }
}
