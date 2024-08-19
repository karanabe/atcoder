use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        q: usize,
    }

    let mut bag = HashMap::new();
    let mut results = Vec::new();

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        match query_type {
            1 => {
                input! { x: usize }
                *bag.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! { x: usize }
                if let Some(count) = bag.get_mut(&x) {
                    *count -= 1;
                    if *count == 0 {
                        bag.remove(&x);
                    }
                }
            }
            3 => {
                results.push(bag.len());
            }
            _ => unreachable!(),
        }
    }

    for result in results {
        println!("{}", result);
    }
}
