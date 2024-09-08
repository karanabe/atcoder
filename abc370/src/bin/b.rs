use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut matrix = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            row: [Usize1; matrix.len() + 1],
        }
        matrix.push(row);
    }

    let mut curr_ele = 0;

    for i in 1..=n {
        if curr_ele >= i {
            curr_ele = matrix[curr_ele][i - 1];
        } else {
            curr_ele = matrix[i - 1][curr_ele];
        }
    }

    println!("{}", curr_ele + 1);
}
