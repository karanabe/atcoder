use proconio::input;

fn main() {
    input! {
        l: i32,
        r: i32,
    }

    match (l, r) {
        (1, 0) => println!("Yes"),
        (0, 1) => println!("No"),
        _ => println!("Invalid"),
    }
}
