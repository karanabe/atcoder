// https://atcoder.jp/contests/abc339/tasks/abc339_c
// C - Perfect Bus
use proconio::input;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Player,
    Obstacle,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            'P' => Cell::Player,
            '#' => Cell::Obstacle,
            _ => panic!("Invalid cell character"),
        }
    }
}
fn main() {
    input! {
        n: usize,
        a: [String; n],
    }
    let _: String = solve(n, a);
}

fn solve(n: usize, a: Vec<String>) -> String {
    let mut grid = vec![];
    let mut players = vec![];

    // グリッドとプレイヤーの位置を読み込む
    for i in 0..n { // 例として5x5グリッド
        let cells: Vec<Cell> = a[i].trim().chars().map(Cell::from_char).collect();

        for j in 0..cells.len() {
            if cells[j] == Cell::Player {
                players.push((i as isize, j as isize));
            }
        }

        grid.push(cells);
    }

    let moves = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    // BFSで最小操作回数を求める
    let mut queue = VecDeque::new();

    // 初期状態をキューに追加
    queue.push_back((players[0], players[1], 0));

    while let Some((p1, p2, steps)) = queue.pop_front() {
        if p1 == p2 {
            println!("{}", steps);
            return format!("OK");
        }

        for &(dx, dy) in &moves {
            let np1 = (p1.0 + dx, p1.1 + dy);
            let np2 = (p2.0 + dx, p2.1 + dy);

            // 移動先のマスが存在し、かつ空きマスであるかどうかをチェック
            let valid = |p: (isize, isize)| {
                p.0 >= 0 && p.0 < grid.len() as isize
                && p.1 >= 0 && p.1 < grid[0].len() as isize
                && grid[p.0 as usize][p.1 as usize] != Cell::Obstacle
            };

            // 移動可能ならばキューに追加
            if valid(np1) && valid(np2) {
                queue.push_back((np1, np2, steps + 1));
            }
        }
    }

    // キューが空になった場合は-1を出力
    println!("-1");
    format!("test")
}

#[cfg(test)]
mod abc339 {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        let a: Vec<String> = vec!["....#".to_string(), "#..#.".to_string(), ".P...".to_string(), "..P..".to_string(), "....#".to_string()];
        assert_eq!("3", solve(n, a));
    }
}
