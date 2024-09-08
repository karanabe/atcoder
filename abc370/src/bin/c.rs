use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut steps = Vec::new();
    let mut current = s.clone();


    for i in 0..s.len() {
        if s[i] != t[i] && s[i] > t[i] {
            current[i] = t[i];
            s[i] = t[i];
            steps.push(current.clone());
        }
    }

    for i in (0..s.len()).rev() {
        if s[i] != t[i] {
            current[i] = t[i];
            s[i] = t[i];
            steps.push(current.clone());
        }
    }

    println!("{}", steps.len());
    for step in steps.iter().map(|vec| vec.iter().collect::<String>()) {
        println!("{}", step);
    }
}
