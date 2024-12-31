use proconio::input;

fn main() {
    input! {
        xyr: [(f64, f64, f64); 2]
    }

    let ab = ((xyr[0].0 - xyr[1].0).powf(2.0) + (xyr[0].1 - xyr[1].1).powf(2.0)).sqrt();

    let r2 = xyr[0].2 + xyr[1].2;
    let r2_abs = (xyr[0].2 - xyr[1].2).abs();

    let answer = if ab < r2_abs {
        1
    } else if ab == r2_abs {
        2
    } else if ab < r2 {
        3
    } else if ab == r2 {
        4
    } else {
        5
    };
    println!("{}", answer);
}
