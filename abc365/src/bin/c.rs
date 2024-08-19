use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }

    let max_x = match find_max_x(n, m, &a) {
        Some(x) => x.to_string(),
        None => "infinite".to_string(),
    };

    println!("{}", max_x);
}

fn find_max_x(n: usize, m: i64, a: &[i64]) -> Option<i64> {
    let mut low = 0;
    let mut high = *a.iter().max().unwrap();

    while low <= high {
        let mid = low + (high - low) / 2;
        if is_feasible(mid, m, a) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    if is_feasible(high, m, a) {
        if high == *a.iter().max().unwrap() && total_cost(high, a) <= m {
            None
        } else {
            Some(high)
        }
    } else {
        Some(high - 1)
    }
}

fn is_feasible(x: i64, m: i64, a: &[i64]) -> bool {
    total_cost(x, a) <= m
}

fn total_cost(x: i64, a: &[i64]) -> i64 {
    a.iter().map(|&cost| std::cmp::min(x, cost)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 4;
        let m = 8;
        let a = vec![1, 3, 2, 4];
        assert_eq!(find_max_x(n, m, &a), Some(2));
    }

    #[test]
    fn test_case_2() {
        let n = 3;
        let m = 20;
        let a = vec![5, 3, 2];
        assert_eq!(find_max_x(n, m, &a), None); // infinite
    }

    #[test]
    fn test_case_3() {
        let n = 10;
        let m = 23;
        let a = vec![2, 5, 6, 5, 2, 1, 7, 9, 7, 2];
        assert_eq!(find_max_x(n, m, &a), Some(2));
    }

    #[test]
    fn test_case_4() {
        let n = 5;
        let m = 15;
        let a = vec![3, 3, 3, 3, 3];
        assert_eq!(find_max_x(n, m, &a), Some(3));
    }

    #[test]
    fn test_case_5() {
        let n = 5;
        let m = 14;
        let a = vec![3, 3, 3, 3, 3];
        assert_eq!(find_max_x(n, m, &a), Some(2));
    }
}
