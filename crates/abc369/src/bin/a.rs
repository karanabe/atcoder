use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let mut results = std::collections::HashSet::new();

    let x1 = 2 * b - a;
    results.insert(x1);

    if (a + b) % 2 == 0 {
        let x2 = (a + b) / 2;
        results.insert(x2);
    }

    let x3 = 2 * a - b;
    results.insert(x3);

    println!("{}", results.len());
}
