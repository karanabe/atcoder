// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ae
// 033 - Distance
use proconio::input;

fn main() {
    input! {
        (ax, ay): (i64, i64),
        (bx, by): (i64, i64),
        (cx, cy): (i64, i64)
    }

    let result = solve((ax, ay), (bx, by), (cx, cy));
    println!("{:.12}", result);
}

fn solve((ax, ay): (i64, i64), (bx, by): (i64, i64), (cx, cy): (i64, i64)) -> f64 {
    let ba_x = ax - bx;
    let ba_y = ay - by;

    let bc_x = cx - bx;
    let bc_y = cy - by;

    let ca_x = ax - cx;
    let ca_y = ay - cy;

    let cb_x = bx - cx;
    let cb_y = by - cy;

    let mut pattern = 2;
    if ba_x * bc_x + ba_y * bc_y < 0 {
        pattern = 1;
    }
    if ca_x * cb_x + ca_y * cb_y < 0 {
        pattern = 3;
    }

    let mut answer: f64 = 0.0;

    if pattern == 1 {
        answer = ((ba_x * ba_x + ba_y * ba_y) as f64).sqrt();
    }
    if pattern == 3 {
        answer = ((ca_x * ca_x + ca_y * ca_y) as f64).sqrt();
    }
    if pattern == 2 {
        let s = (ba_x * ca_y - ba_y * ca_x).abs() as f64;
        let bc_length = ((bc_x * bc_x + bc_y * bc_y) as f64).sqrt();
        answer = s / bc_length;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = solve((0, 5), (1, 1), (3, 0));
        let expected = 4.123105625618;
        assert!((result - expected).abs() < 1e-12);
    }

    #[test]
    fn test_case_2() {
        let result = solve((-40, -30), (-50, -10), (-20, -20));
        let expected = 15.811388300842;
        assert!((result - expected).abs() < 1e-12);
    }

    #[test]
    fn test_case_3() {
        let result = solve((1000000000, 1000000000), (-1000000000, -1000000000), (0, -1000000000));
        let expected = 2236067977.499789714813;
        assert!((result - expected).abs() < 1e-12);
    }
}
