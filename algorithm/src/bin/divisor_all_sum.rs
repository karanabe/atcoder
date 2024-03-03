// Calc divisor and sum all numbers

fn main() {
    let n: usize = 10;
    let _: usize = solve(n);
}

fn solve(n: usize) -> usize {
    let mut result = 0;
    for i in 1..=n {
        if n % i == 0 { result += i; }
    }
    return result;
}


#[cfg(test)]
mod divisor {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 6;
        assert_eq!(12, solve(n));
    }

    #[test]
    fn test_2() {
        let n: usize = 11;
        assert_eq!(12, solve(n));
    }

    #[test]
    fn test_3() {
        let n: usize = 23;
        assert_eq!(24, solve(n));
    }

    #[test]
    fn test_4() {
        let n: usize = 143;
        assert_eq!(168, solve(n));
    }

    #[test]
    fn test_5() {
        let n: usize = 203;
        assert_eq!(240, solve(n));
    }
}
