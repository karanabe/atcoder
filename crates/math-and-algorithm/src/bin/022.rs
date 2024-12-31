// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_v
// 022 - Choose Cards 3
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(n, a);
}

fn solve(_n: usize, a: Vec<usize>) -> usize {
    let mut result = 0;
    let mut map = BTreeMap::new();
    // Count like histogram
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    // Get values less than 50000 from the list
    for (key, value) in map.range(..50000) {
        result += value * map.get(&(100000-key)).unwrap_or(&0);
    }
    // Count 50000
    // nC2 => n is 50000 count
    let n = map.get(&50000).unwrap_or(&0);
    result += n * (n - 1) / 2;
    println!("{result}");
    result
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 6;
        let a = vec![40000, 50000, 20000, 80000, 50000, 30000];
        assert_eq!(2, solve(n, a));
    }

    #[test]
    fn test_2() {
        let n = 12;
        let a = vec![40000, 50000, 20000, 80000, 50000, 30000, 40000, 50000, 20000, 80000, 50000, 30000];
        assert_eq!(10, solve(n, a));
    }
}
