use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
    }

    let mut ans = 0;
    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            ans += 1;
        }
    }

    let mut xi = Vec::with_capacity(q);
    let mut ci = Vec::with_capacity(q);
    for _ in 0..q {
        input! {
            x: usize,
            c: char,
        }
        xi.push(x - 1);
        ci.push(c);
    }

    for (&x, &c) in xi.iter().zip(ci.iter()) {
        for i in x.saturating_sub(2)..=(x + 2).min(n - 1) {
            if i + 2 < n {
                if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                    ans -= 1;
                }
            }
        }

        s[x] = c;

        for i in x.saturating_sub(2)..=(x + 2).min(n - 1) {
            if i + 2 < n {
                if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                    ans += 1;
                }
            }
        }

        println!("{}", ans);
    }
}
