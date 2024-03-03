// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aa
// 027 - Sorting
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let _: String = solve(n, a);
}

fn solve(_n: usize, mut a: Vec<usize>) -> String {

    /*
    for i in 0..n-1 {
        let mut min_index = i;
        let mut min_value = a[i];
        for j in i..n {
            if a[j] < min_value {
                min_index = j;
                min_value = a[j];
            }
        }
        a.swap(i, min_index);
    }
    */

    merge_sort(&mut a);

    let result: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    let ans = result.join(" ");
    println!("{ans}");


    format!("{ans}")
}

fn merge_sort<T: Ord + Copy>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }
    let length = data.len();
    let middle = length / 2;
    merge_sort(&mut data[..middle]);
    merge_sort(&mut data[middle..]);
    let mut work_vec: Vec<T> = Vec::with_capacity(length);
    for i in 0..length {
        work_vec.push(data[i]);
    }
    work_vec[middle..].reverse();
    let mut l = 0;
    let mut r = length - 1;
    for i in 0..length {
        if work_vec[l] > work_vec[r] {
            data[i] = work_vec[r];
            r -= 1;
        } else {
            data[i] = work_vec[l];
            l += 1;
        }
    }
}


#[cfg(test)]
mod maa {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let a = vec![3, 1, 2];
        assert_eq!("1 2 3", solve(n, a));
    }

    #[test]
    fn test_2() {
        let n = 10;
        let a = vec![658, 299, 47, 507, 122, 969, 449, 68, 513, 800];
        assert_eq!("47 68 122 299 449 507 513 658 800 969", solve(n, a));
    }
}
