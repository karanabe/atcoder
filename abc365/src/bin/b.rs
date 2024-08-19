//
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let _: usize = solve(n, a);
}

fn solve(_n: usize, a: Vec<i64>) -> usize {
    let mut indexed_a: Vec<(i64, usize)> = a.iter().copied().enumerate().map(|(i, x)| (x, i + 1)).collect();
    indexed_a.sort_by(|a, b| b.0.cmp(&a.0));

    println!("{}", indexed_a[1].1);
    indexed_a[1].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 4;
        let a = vec![8, 2, 5, 1];
        assert_eq!(find_second_largest_index(n, a), 3);
    }

    #[test]
    fn test_case_2() {
        let n = 8;
        let a = vec![1, 2, 3, 4, 5, 10, 9, 11];
        assert_eq!(find_second_largest_index(n, a), 6);
    }

    fn find_second_largest_index(_n: usize, a: Vec<i64>) -> usize {
        let mut indexed_a: Vec<(i64, usize)> = a.iter().copied().enumerate().map(|(i, x)| (x, i + 1)).collect();
        indexed_a.sort_by(|a, b| b.0.cmp(&a.0));
        indexed_a[1].1
    }
}

