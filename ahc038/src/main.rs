#[allow(unused_imports)]
use proconio::{
    input,
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
    Reverse
};

#[allow(unused_imports)]
use std::time::{Instant, Duration};
#[allow(unused_imports)]
use rand::prelude::*;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    input! {
        n: usize,
        m: usize,
        v: usize,
    }

    let mut ans_queue: BinaryHeap<Reverse<(usize, VecDeque<Vec<char>>, usize, usize)>> = BinaryHeap::new();
    //let mut action_queue = VecDeque::new();

    let mut s_grid = Vec::new();
    for _ in 0..n {
        input! {
            s_row: String,
        }
        s_grid.push(s_row.chars().collect::<Vec<char>>());
    }
    let mut t_grid = Vec::new();
    for _ in 0..n {
        input! {
            t_row: String,
        }
        t_grid.push(t_row.chars().collect::<Vec<char>>());
    }

    let mut takoyaki = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if s_grid[i][j] == '1' {
                takoyaki.insert((i, j));
            }
        }
    }

    let mut target_positions = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if t_grid[i][j] == '1' {
                target_positions.insert((i, j));
            }
        }
    }

    let common_elements: HashSet<(usize, usize)> = takoyaki.intersection(&target_positions).cloned().collect();

    for elem in &common_elements {
        takoyaki.remove(elem);
    }

    for elem in &common_elements {
        target_positions.remove(elem);
    }

    let takoyaki_org = takoyaki.clone();
    let target_positions_org = target_positions.clone();


    let start = Instant::now();
    let time_limit = Duration::from_millis(2000);
    let mut _temp = 1000.0;
    let _temp_min = 1e-4;
    let _cooling_rate = 0.995;

    let v_prime = 5;
    println!("{}", v_prime);
    println!("0 2");
    println!("1 1");
    println!("1 1");
    println!("1 1");
    let mut x = 0;
    let mut y = 0;

    let mut start_pos = generate_random_coordinates(n);

    let mut dir_current = 0;
    let mut holding = false;
    let mut holding_1 = false;
    let mut holding_2 = false;

    let mut _arm_direction = 0;
    let mut only_first_count = 0;

    let mut calc_count = 0;

    let dirs = vec![(0, 1, 'R'), (1, 0, 'D'), (0, !0, 'L'), (!0, 0, 'U')];
    let arms = vec![
        [(2, 1), (3, 0), (2, -1)],
        [(1, -2), (0, -3), (-1, -2)],
        [(-2, -1), (-3, 0), (-2, 1)],
        [(-1, 2), (0, 3), (1, 2)]
    ];

    while start.elapsed() < time_limit  {
        //&& temp > temp_min

        if start_pos.len() != 0 {
            if calc_count != 0 {
                (x, y) = start_pos.pop().unwrap();
            }
        } else {
            break;
        }

        let mut root_x = x;
        let mut root_y = y;

        dir_current = 0;
        only_first_count = 0;
        holding = false;
        holding_1 = false;
        holding_2 = false;

        let mut action_queue = VecDeque::new();
        let mut takoyaki = takoyaki_org.clone();
        let mut target_positions = target_positions_org.clone();

        // println!("{:?}", target_positions);

        while target_positions.len() != 0 {
            // println!("{} {}", takoyaki.len(), target_positions.len());
            let start_pos = nearest_point(root_x, root_y, &mut takoyaki).unwrap();
            let (ssx, ssy) = start_pos;
            //println!("{} {}", root_x, root_x);
            //println!("{} {}", ssx, ssy);
            let (sx, sy) = nearest_goal_in_grid(root_x, root_y, ssx, ssy, n).unwrap();


            let target_pos_option = nearest_point(sx, sy, &mut target_positions).unwrap();
            let (ttx, tty) = target_pos_option;
            let (tx, ty) = nearest_goal_in_grid(sx, sy, ttx, tty, n).unwrap();

            let start_direction = goal_direction(sx, sy, ssx, ssy);
            //println!("{start_direction}");

            while (root_x as isize - sx as isize).abs() + (root_y as isize - sy as isize).abs() != 0 {
                let mut cmd = vec!['.'; v_prime*2];
                let dx = sx as isize - root_x as isize;
                let dy = sy as isize - root_y as isize;
                if dx != 0 {
                    let step = if dx > 0 { 1 } else { -1 };
                    root_x = (root_x as isize + step) as usize;
                    cmd[0] = if step == 1 { 'D' } else { 'U' };
                } else if dy != 0 {
                    let step = if dy > 0 { 1 } else { -1 };
                    root_y = (root_y as isize + step) as usize;
                    cmd[0] = if step == 1 { 'R' } else { 'L' };
                }

                if only_first_count != 1 {
                    cmd[3] = 'R';
                    cmd[4] = 'L';
                    only_first_count += 1;
                }

                if dir_current != start_direction {
                    let diff = (start_direction + 4 - dir_current) % 4;
                    if diff == 1 {
                        cmd[1] = 'R';
                        dir_current = (dir_current + 1) % 4;
                    } else {
                        cmd[1] = 'L';
                        dir_current = (dir_current + 3) % 4;
                    }
                    // println!("{}", cmd.iter().collect::<String>());
                    // action_queue.push_back(cmd);
                }

                pick_or_drop_takoyaki(
                    n,
                    root_x,
                    root_y,
                    &arms,
                    dir_current,
                    &mut takoyaki,
                    &mut target_positions,
                    &mut holding_1,
                    &mut holding_2,
                    sx,
                    sy,
                    tx,
                    ty,
                    &mut cmd
                );

                // println!("{}", cmd.iter().collect::<String>());
                action_queue.push_back(cmd);
            }
            while dir_current != start_direction {
                let mut cmd = vec!['.'; v_prime*2];
                let diff = (start_direction + 4 - dir_current) % 4;
                if diff == 1 {
                    cmd[1] = 'R';
                    dir_current = (dir_current + 1) % 4;
                } else {
                    cmd[1] = 'L';
                    dir_current = (dir_current + 3) % 4;
                }

                pick_or_drop_takoyaki(
                    n,
                    root_x,
                    root_y,
                    &arms,
                    dir_current,
                    &mut takoyaki,
                    &mut target_positions,
                    &mut holding_1,
                    &mut holding_2,
                    sx,
                    sy,
                    tx,
                    ty,
                    &mut cmd
                );

                // println!("{}", cmd.iter().collect::<String>());
                action_queue.push_back(cmd);
            }

            let mut cmd = vec!['.'; v_prime*2];
            cmd[7] = 'P';
            holding = true;
            // println!("{}", cmd.iter().collect::<String>());
            action_queue.push_back(cmd);

            let end_direction = goal_direction(tx, ty, ttx, tty);

            while (root_x as isize - tx as isize).abs() + (root_y as isize - ty as isize).abs() != 0 {
                let mut cmd = vec!['.'; v_prime*2];

                let dx = tx as isize - root_x as isize;
                let dy = ty as isize - root_y as isize;
                if dx != 0 {
                    let step = if dx > 0 { 1 } else { -1 };
                    root_x = (root_x as isize + step) as usize;
                    cmd[0] = if step == 1 { 'D' } else { 'U' };
                } else if dy != 0 {
                    let step = if dy > 0 { 1 } else { -1 };
                    root_y = (root_y as isize + step) as usize;
                    cmd[0] = if step == 1 { 'R' } else { 'L' };
                }

                if dir_current != end_direction {
                    let diff = (end_direction + 4 - dir_current) % 4;
                    if diff == 1 {
                        cmd[1] = 'R';
                        dir_current = (dir_current + 1) % 4;
                    } else {
                        cmd[1] = 'L';
                        dir_current = (dir_current + 3) % 4;
                    }
                    // println!("{}", cmd.iter().collect::<String>());
                    // action_queue.push_back(cmd);
                }

                pick_or_drop_takoyaki(
                    n,
                    root_x,
                    root_y,
                    &arms,
                    dir_current,
                    &mut takoyaki,
                    &mut target_positions,
                    &mut holding_1,
                    &mut holding_2,
                    sx,
                    sy,
                    tx,
                    ty,
                    &mut cmd
                );

                // println!("{}", cmd.iter().collect::<String>());
                action_queue.push_back(cmd);
            }

            while dir_current != end_direction {
                let mut cmd = vec!['.'; v_prime*2];
                let diff = (end_direction + 4 - dir_current) % 4;
                if diff == 1 {
                    cmd[1] = 'R';
                    dir_current = (dir_current + 1) % 4;
                } else {
                    cmd[1] = 'L';
                    dir_current = (dir_current + 3) % 4;
                }

                pick_or_drop_takoyaki(
                    n,
                    root_x,
                    root_y,
                    &arms,
                    dir_current,
                    &mut takoyaki,
                    &mut target_positions,
                    &mut holding_1,
                    &mut holding_2,
                    sx,
                    sy,
                    tx,
                    ty,
                    &mut cmd
                );

                // println!("{}", cmd.iter().collect::<String>());
                action_queue.push_back(cmd);
            }
            let mut cmd = vec!['.'; v_prime*2];
            cmd[7] = 'P';
            holding = false;
            // println!("{}", cmd.iter().collect::<String>());
            action_queue.push_back(cmd);
        }

        let mut sa_queue = VecDeque::new();
        let mut current_cmd = vec!['.'; v_prime*2];
        while let Some(next_cmd) = action_queue.pop_front() {
            let mut can_merge = true;
            for i in 0..=7 {
                if current_cmd[7] == 'P' {
                    can_merge = false;
                    break;
                } else if current_cmd[i] == '.' {
                    current_cmd[i] = next_cmd[i];
                } else if next_cmd[i] == '.' {
                } else {
                    can_merge = false;
                    break;
                }
            }
            if !can_merge {
                sa_queue.push_back(current_cmd);
                //println!("{}", current_cmd.iter().collect::<String>());
                current_cmd = next_cmd;
            }
        }
        sa_queue.push_back(current_cmd);
        //println!("{}", current_cmd.iter().collect::<String>());

        ans_queue.push(Reverse((sa_queue.len(), sa_queue, x, y)));
        calc_count += 1;
    }

    /*let mut keys: Vec<&usize> = ans_queue.keys().collect();
    keys.sort_by(|a, b| b.cmp(a));
    for x in keys {
        println!("{x}");
    }*/

    if let Some(Reverse((min_len, min_queue, x, y))) = ans_queue.pop() {
        println!("{} {}", x, y);

        for vec in min_queue {
            println!("{}", vec.iter().collect::<String>());
        }
    }

}


fn nearest_point(root_x: usize, root_y: usize, points: &mut HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    if points.is_empty() {
        return None;
    }

    let mut min_distance = std::isize::MAX;
    let mut closest_point = None;

    for &(x, y) in points.iter() {
        let distance = (root_x as isize - x as isize).abs() + (root_y as isize - y as isize).abs();

        if distance < min_distance {
            min_distance = distance;
            closest_point = Some((x, y));
        }
    }

    if let Some(point) = closest_point {
        points.take(&point);
    }

    closest_point
}


fn nearest_goal_in_grid(root_x: usize, root_y: usize, goal_x: usize, goal_y: usize, n: usize) -> Option<(usize, usize)> {
    let mut goals = HashSet::new();
    let root_len = 3;
    if goal_x >= root_len { goals.insert((goal_x - root_len, goal_y)); }
    if goal_y >= root_len { goals.insert((goal_x, goal_y - root_len)); }
    if goal_x + root_len < n { goals.insert((goal_x + root_len, goal_y)); }
    if goal_y + root_len < n { goals.insert((goal_x, goal_y + root_len)); }

    if goals.is_empty() {
        return None;
    }

    let mut min_distance = std::isize::MAX;
    let mut closest_goal = None;

    for &(gx, gy) in goals.iter() {
        let distance = (root_x as isize - gx as isize).abs() + (root_y as isize - gy as isize).abs();

        if distance < min_distance {
            min_distance = distance;
            closest_goal = Some((gx, gy));
        }
    }

    closest_goal
}

fn goal_direction(root_x: usize, root_y: usize, goal_x: usize, goal_y: usize) -> usize {
    if goal_x > root_x {
        return 1;
    } else if goal_x < root_x {
        return 3;
    } else if goal_y > root_y {
        return 0;
    } else {
        //goal_y < root_y
        return 2;
    }
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn pick_or_drop_takoyaki(
    n: usize,
    root_x: usize,
    root_y: usize,
    arms: &Vec<[(isize, isize); 3]>,
    direction: usize,
    takoyaki: &mut HashSet<(usize, usize)>,
    target_positions: &mut HashSet<(usize, usize)>,
    holding_1: &mut bool,
    holding_2: &mut bool,
    sx: usize,
    sy: usize,
    gx: usize,
    gy: usize,
    cmd: &mut Vec<char>
) {
    return;
    #[allow(unreachable_code)]
    let arm_positions = arms[direction];

    let arm_pos_1 = (root_x as isize + arm_positions[0].0, root_y as isize + arm_positions[0].1);
    let arm_pos_2 = (root_x as isize + arm_positions[2].0, root_y as isize + arm_positions[2].1);

    let arm_pos_1u: (usize, usize);
    let arm_pos_2u: (usize, usize);

    if is_in_grid(arm_pos_1.0, arm_pos_1.1, n) {
        arm_pos_1u = ((root_x as isize + arm_positions[0].0).abs() as usize, (root_y as isize + arm_positions[0].1).abs() as usize);
    } else {
        arm_pos_1u = (10000usize, 10000usize);
    }

    if is_in_grid(arm_pos_2.0, arm_pos_2.1, n) {
        arm_pos_2u = ((root_x as isize + arm_positions[2].0).abs() as usize, (root_y as isize + arm_positions[2].1).abs() as usize);
    } else {
        arm_pos_2u = (10000usize, 10000usize);
    }

    if !*holding_1 {
        if takoyaki.take(&arm_pos_1u).is_some() && (sx, sy) != arm_pos_1u {
            takoyaki.take(&arm_pos_1u);
            cmd[8] = 'P';
            *holding_1 = true;
        }
    } else {
        if target_positions.contains(&arm_pos_1u) && (gx, gy) != arm_pos_1u {
            target_positions.take(&arm_pos_1u);
            cmd[8] = 'P';
            *holding_1 = false;
        }
    }

    if !*holding_2 {
        if takoyaki.take(&arm_pos_2u).is_some() && (sx, sy) != arm_pos_2u {
            takoyaki.take(&arm_pos_2u);
            cmd[9] = 'P';
            *holding_2 = true;
        }
    } else {
        if target_positions.contains(&arm_pos_2u) && (gx, gy) != arm_pos_2u {
            target_positions.take(&arm_pos_2u);
            cmd[9] = 'P';
            *holding_2 = false;
        }
    }
}

fn is_in_grid(x: isize, y: isize, n: usize) -> bool {
    x >= 3 && y >= 3 && (x as usize - 3) < n && (y as usize - 3) < n
}

fn generate_random_coordinates(n: usize) -> Vec<(usize, usize)> {
    let mut coordinates: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            coordinates.push((i, j));
        }
    }

    let mut rng = thread_rng();
    coordinates.shuffle(&mut rng);

    coordinates
}
