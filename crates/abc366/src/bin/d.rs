use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[[i64; n]; n]; n],
        q: usize,
        queries: [(usize, usize, usize, usize, usize, usize); q],
    }

    let mut prefix_sum = vec![vec![vec![0i64; n + 1]; n + 1]; n + 1];

    for x in 1..=n {
        for y in 1..=n {
            for z in 1..=n {
                prefix_sum[x][y][z] = a[x-1][y-1][z-1]
                    + prefix_sum[x-1][y][z]
                    + prefix_sum[x][y-1][z]
                    + prefix_sum[x][y][z-1]
                    - prefix_sum[x-1][y-1][z]
                    - prefix_sum[x-1][y][z-1]
                    - prefix_sum[x][y-1][z-1]
                    + prefix_sum[x-1][y-1][z-1];
            }
        }
    }

    let mut results = Vec::with_capacity(q);

    for (lx, rx, ly, ry, lz, rz) in queries {
        let sum = prefix_sum[rx][ry][rz]
            - prefix_sum[lx-1][ry][rz]
            - prefix_sum[rx][ly-1][rz]
            - prefix_sum[rx][ry][lz-1]
            + prefix_sum[lx-1][ly-1][rz]
            + prefix_sum[lx-1][ry][lz-1]
            + prefix_sum[rx][ly-1][lz-1]
            - prefix_sum[lx-1][ly-1][lz-1];

        results.push(sum);
    }

    for result in results {
        println!("{}", result);
    }
}
