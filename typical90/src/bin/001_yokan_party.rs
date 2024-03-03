// https://atcoder.jp/contests/typical90/tasks/typical90_a
use proconio::input;

fn main() {
    input! {
        n: usize, l: usize,
        k: usize,
        a: [usize; n],
    }

    println!("n:{} l:{} k:{} a:{:?}", n, l, k, a);
}

fn solve(N:usize, l: usize, k: usize, a: &[usize]) {
    let mut tmp: usize = 0;
    let mut cut = Vec::new();
    for (index, &arr) in a.into_iter().enumerate() {
        if tmp == 0 {
            println!("index:{} arr:{}", index, arr);
            cut.push(arr);
            tmp = 1;
        } else {
            let yokan_pice = arr - a[index-1];
            println!("index:{} arr:{} pice:{}", index, arr, yokan_pice);
            cut.push(yokan_pice);
        }
    }
    let yokan_pice = l - a[N-1];
    cut.push(yokan_pice);

    println!("cut:{:?}", cut);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const N: usize = 3;
        let l: usize = 34;
        let k: usize = 1;
        let a: [usize; N] = [8, 13, 26];
        solve(N, l, k, &a);
        assert_eq!(2 + 2, 4);
    }
}
