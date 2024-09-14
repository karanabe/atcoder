use proconio::input;

fn lower_bound(arr: &Vec<i64>, x: i64) -> usize {
    let mut low = 0usize;
    let mut high = arr.len();
    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] < x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

fn upper_bound(arr: &Vec<i64>, x: i64) -> usize {
    let mut low = 0usize;
    let mut high = arr.len();
    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] <= x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

fn main() {
    input! {
        n: usize,
        xi: [i64; n],
        pi: [u64; n],
        q: usize,
    }

    let mut ps = vec![0u64; n + 1];
    for i in 0..n {
        ps[i + 1] = ps[i] + pi[i];
    }

    for _ in 0..q {
        input! {
            li: i64,
            ri: i64,
        }
        let l = lower_bound(&xi, li);
        let r = upper_bound(&xi, ri);
        let ans = ps[r] - ps[l];
        println!("{}", ans);
    }
}
