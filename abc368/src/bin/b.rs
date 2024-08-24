use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    let mut ops = 0;

    loop {
        a.sort_by(|x, y| y.cmp(x));

        if a[0] == 0 || a[1] == 0 {
            break;
        }

        a[0] -= 1;
        a[1] -= 1;

        ops += 1;
    }

    println!("{}", ops);
}
