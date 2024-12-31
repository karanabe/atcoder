// https://atcoder.jp/contests/abc355/tasks/abc355_b
// B - Piano 2
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    }
    let _: String = solve(n, m, a, b);
}

fn solve(_n: usize, _m: usize, a: Vec<usize>, mut b: Vec<usize>) -> String {
    let mut numbers = a.clone();
    numbers.append(&mut b);
    numbers.sort();

    let mut result = false;

    for item in &numbers {
        if a.contains(&item) {
            if result {
                println!("Yes");
                return format!("Yes");
            } else {
                result = true;
            }
        } else {
            result = false;
        }
    }

    println!("No");
    format!("No")
}


#[cfg(test)]
mod abc355 {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let m = 2;
        let a = vec![3, 2, 5];
        let b = vec![4, 1];
        assert_eq!("Yes", solve(n, m, a, b));
    }

    #[test]
    fn test_2() {
        let n = 3;
        let m = 2;
        let a = vec![3, 1, 5];
        let b = vec![4, 2];
        assert_eq!("No", solve(n, m, a, b));
    }

    #[test]
    fn test_3() {
        let n = 1;
        let m = 1;
        let a = vec![1];
        let b = vec![2];
        assert_eq!("No", solve(n, m, a, b));
    }

    #[test]
    fn test_4() {
        let n = 1;
        let m = 2;
        let a = vec![1];
        let b = vec![2, 3];
        assert_eq!("No", solve(n, m, a, b));
    }

    #[test]
    fn test_5() {
        let n = 2;
        let m = 10;
        let a = vec![8, 9];
        let b = vec![1, 2, 3, 4, 5, 6, 7, 10, 11, 12];
        assert_eq!("Yes", solve(n, m, a, b));
    }

    #[test]
    fn test_6() {
        let n = 2;
        let m = 1;
        let a = vec![45, 49];
        let b = vec![40];
        assert_eq!("Yes", solve(n, m, a, b));
    }
}
