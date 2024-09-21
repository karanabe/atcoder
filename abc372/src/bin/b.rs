use proconio::input;

fn main() {
    input! {
        mut m: usize,
    }

    let mut digits = Vec::new();

    while m > 0 {
        digits.push(m % 3);
        m /= 3;
    }

    let mut a = Vec::new();
    for (i, &digit) in digits.iter().enumerate() {
        for _ in 0..digit {
            a.push(i);
        }
    }

    let n = a.len();

    println!("{}", n);

    for (i, ai) in a.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ai);
    }
    println!();
}
