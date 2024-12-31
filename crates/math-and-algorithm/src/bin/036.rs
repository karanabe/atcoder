// 極座標での2点間の距離
// 余弦定理
use proconio::input;
const PI:f64 = std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let t1 = (h + m / 60.0) * PI / 6.0;

    let t2 = m * PI / 30.0;

    let ans = (a * a + b * b - 2.0 * a * b * (t1 - t2).cos()).sqrt();
    print!("{}", ans);
}

