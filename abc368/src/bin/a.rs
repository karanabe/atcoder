use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
    }

    let mut b = Vec::new();
    b.extend_from_slice(&a[n-k..n]);
    b.extend_from_slice(&a[0..n-k]);

    for (i, x) in b.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", x);
    }
    println!();
}
