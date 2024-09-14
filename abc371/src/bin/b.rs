use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        babies: [(usize, char); m],
    }

    let mut first_male_born = vec![false; n + 1];

    for &(ai, bi) in &babies {
        if bi == 'M' {
            if !first_male_born[ai] {
                println!("Yes");
                first_male_born[ai] = true;
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}
