use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        points: [(i64, i64); n],
    }

    let mut x_coords: Vec<i64> = points.iter().map(|&(x, _)| x).collect();
    let mut y_coords: Vec<i64> = points.iter().map(|&(_, y)| y).collect();

    x_coords.sort();
    y_coords.sort();

    let mut fx = vec![0; n];
    let mut gx = vec![0; n];

    for (i, &x_i) in x_coords.iter().enumerate() {
        fx[i] = x_coords.iter().map(|&x_j| (x_i - x_j).abs()).sum();
    }

    for (i, &y_i) in y_coords.iter().enumerate() {
        gx[i] = y_coords.iter().map(|&y_j| (y_i - y_j).abs()).sum();
    }

    let mut count = 0;
    let mut j = 0;

    for &fx_i in fx.iter() {
        while j < n && fx_i + gx[j] <= d {
            j += 1;
        }
        count += j;
    }

    println!("{}", count);
}
