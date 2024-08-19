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

    for i in 0..n {
        fx[i] = (0..n).map(|j| (x_coords[i] - x_coords[j]).abs()).sum();
    }

    for i in 0..n {
        gx[i] = (0..n).map(|j| (y_coords[i] - y_coords[j]).abs()).sum();
    }

    let mut count = 0;
    let mut j = 0;

    for i in 0..n {
        while j < n && fx[i] + gx[j] <= d {
            j += 1;
        }
        count += j;
    }

    println!("{}", count);
}
