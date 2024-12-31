use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let result = if b < c {
        if a >= b && a < c {
            "No"
        } else {
            "Yes"
        }
    } else {
        if a >= b || a < c {
            "No"
        } else {
            "Yes"
        }
    };

    println!("{}", result);
}
