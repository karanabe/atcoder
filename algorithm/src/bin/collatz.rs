// Collatz Problem

fn main() {
    let n: usize = 10;
    let _: usize = solve(n);
}

fn solve(mut n: usize) -> usize {
    let mut count: usize = 0;
    loop {
        if n == 1 { break; }
        if n % 2 == 0 { n = n / 2; count += 1; }
        else { n = 3*n + 1; count += 1; }
    }
    println!("n={}, count={}", n, count);
    return n;
}


#[cfg(test)]
mod collatz {
    use super::*;

    #[test]
    fn test_1() {
        let n: usize = 10;
        assert_eq!(1, solve(n));
    }

    #[test]
    fn test_2() {
        let n: usize = 432;
        assert_eq!(1, solve(n));
    }

    #[test]
    fn test_3() {
        let n: usize = 4673;
        assert_eq!(1, solve(n));
    }
}
