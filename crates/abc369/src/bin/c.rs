use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut count = n as i64;
    let mut i = 0;

    while i < n - 1 {
        let mut j = i;
        while j < n - 1 && a[j + 1] - a[j] == a[i + 1] - a[i] {
            j += 1;
        }
        let len = (j - i + 1) as i64;
        count += len * (len - 1) / 2;
        i = j;
    }

    println!("{}", count);
}
