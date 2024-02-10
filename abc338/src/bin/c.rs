// https://atcoder.jp/contests/abc335/tasks/abc338_c
// C - Leftover Recipes
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [i32; n],
        a: [i32; n],
        b: [i32; n],
    }
    let _: String = solve(n, q, a, b);
}

fn solve(n: usize, q: Vec<i32>, a: Vec<i32>, b: Vec<i32>) -> String {

    let ((top_max, _top_min), (bottom_max, _bottom_min)) = find_max_min_positions(&a, &b);

    let top = (q[top_max] - b[top_max]) / a[top_max];
    let bottom = (q[bottom_max] - a[bottom_max]) / b[bottom_max];

    println!("{}", top+bottom);
    format!("{}", top+bottom)
}

fn find_max_min_positions(a: &Vec<i32>, b: &Vec<i32>) -> ((usize, usize), (usize, usize)) {
    let mut max_a_min_b = (0, 0);

    let mut min_a_max_b = (0, 0);

    for i in 0..a.len() {

        if a[i] > a[max_a_min_b.0] || (a[i] == a[max_a_min_b.0] && b[i] < b[max_a_min_b.1]) {
            max_a_min_b = (i, i);
        }

        if a[i] < a[min_a_max_b.0] || (a[i] == a[min_a_max_b.0] && b[i] > b[min_a_max_b.1]) {
            min_a_max_b = (i, i);
        }
    }

    (max_a_min_b, min_a_max_b)
}

#[cfg(test)]
mod abc338 {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 2;
        let q: Vec<i32> = vec![800, 300];
        let a: Vec<i32> = vec![100, 100];
        let b: Vec<i32> = vec![200, 10];
        assert_eq!("5", solve(n, q, a, b));
    }

    #[test]
    fn test_2() {
        let n: usize = 2;
        let q: Vec<i32> = vec![800, 300];
        let a: Vec<i32> = vec![100, 0];
        let b: Vec<i32> = vec![0, 10];
        assert_eq!("38", solve(n, q, a, b));
    }

    #[test]
    fn test_3() {
        let n: usize = 2;
        let q: Vec<i32> = vec![800, 300];
        let a: Vec<i32> = vec![801, 300];
        let b: Vec<i32> = vec![800, 301];
        assert_eq!("0", solve(n, q, a, b));
    }

    #[test]
    fn test_4() {
        let n: usize = 10;
        let q: Vec<i32> = vec![1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000];
        let a: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let b: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!("222222", solve(n, q, a, b));
    }

    #[test]
    fn test_5() {
        let n: usize = 2;
        let q: Vec<i32> = vec![800, 300];
        let a: Vec<i32> = vec![0, 0];
        let b: Vec<i32> = vec![0, 0];
        assert_eq!("5", solve(n, q, a, b));
    }
}
