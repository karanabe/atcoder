// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_t
// 020 - Choose Cards 2
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: usize = solve(n, a);
}

fn solve(n: usize, a: Vec<usize>) -> usize {
    let mut result = 0;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                for l in k+1..n {
                    for m in l+1..n {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            result += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{result}");
    result
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        let a = vec![100, 150, 200, 250, 300];
        assert_eq!(1, solve(n, a));
    }

    #[test]
    fn test_2() {
        let n = 13;
        let a = vec![243, 156, 104, 280, 142, 286, 196, 132, 128, 195, 265, 300, 130];
        assert_eq!(4, solve(n, a));
    }
}
