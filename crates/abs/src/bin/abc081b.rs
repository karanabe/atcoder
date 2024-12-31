// https://atcoder.jp/contests/abs/tasks/abc081_b
use proconio::input;

fn main() {
    input! {
        n: u8,
        a: [u32; n],
    }
    let _: usize = solve(&a);
}

fn solve(a: &[u32]) -> usize {
    let mut div_count: usize = 0;
    let mut v1: Vec<_>  = a.to_vec();
    while odd_check(&v1) {
        div_count += 1;
        v1 = v1.iter().map(|x| x / 2).collect();
    }
    print!("{}", div_count);
    div_count
}

fn odd_check(v1: &[u32]) -> bool {
    for v in v1.iter() {
        if v % 2 == 1 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod abs {
    use super::*;

    #[test]
    fn div_count_test_1() {
        let a: [u32; 3] = [8, 12, 40];
        assert_eq!(2, solve(&a));
    }

    #[test]
    fn div_count_test_2() {
        let a: [u32; 4] = [5, 6, 8, 10];
        assert_eq!(0, solve(&a));
    }

    #[test]
    fn div_count_test_3() {
        let a: [u32; 6] = [382253568, 723152896, 37802240, 379425024, 404894720, 471526144];
        assert_eq!(8, solve(&a));
    }

}
