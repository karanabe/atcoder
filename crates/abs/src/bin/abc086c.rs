// https://atcoder.jp/contests/abs/tasks/arc089_a
use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [[i32; 3]; n],
    }
    let _: String = solve(&a);
}

fn solve(a: &Vec<Vec<i32>>) -> String {
    let mut t: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for v in a {
        let dt = v[0] - t;
        let dist = (v[1] - x).abs() + (v[2] - y).abs();
        if dist > dt { println!("No"); return "No".to_string(); }
        // if ((dt - dist) % 2 == 1) { println!("No"); return "No".to_string(); }
        if dist % 2 != dt % 2 { println!("No"); return "No".to_string(); }

        t = v[0];
        x = v[1];
        y = v[2];
    }
    println!("Yes");
    return "Yes".to_string();
}


#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn test_1() {
        let a: Vec<Vec<i32>> = vec![vec![3, 1, 2], vec![6, 1, 1]];
        assert_eq!("Yes", solve(&a));
    }

    #[test]
    fn test_2() {
        let a: Vec<Vec<i32>> = vec![vec![2, 100, 100]];
        assert_eq!("No", solve(&a));
    }

    #[test]
    fn test_3() {
        let a: Vec<Vec<i32>> = vec![vec![5, 1, 1], vec![100, 1, 1]];
        assert_eq!("No", solve(&a));
    }

}
