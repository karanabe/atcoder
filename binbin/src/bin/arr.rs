fn main() {
    solve();
}

fn solve() {
    let bar = vec![2, 4, 5, 6, 7];

    // n is &i32
    for n in &bar {
        print!("{} ", n);
    }
    println!("");

    // n is &i32
    for n in bar.iter() {
        print!("{} ", n);
    }
    println!("");

    // n is i32
    for n in bar.into_iter() {
        print!("{} ", n);
    }
    println!("");

    // bar can not use after above for loop

    let bar = vec![4, 3, 2, 1, 4];
    let mut custom_bar: Vec<i32> = Vec::with_capacity(bar.len());

    // This case could not use array input. like `custom_bar[index] = *n;`
    for (_index, n) in bar.iter().enumerate() {
        custom_bar.push(*n);
    }
    println!("{:?}", custom_bar);

    let bar = [1, 2, 3, 4, 5, 6, 7];
    let mut custom_bar = [0; 7];

    for (index, n) in bar.iter().enumerate() {
        custom_bar[index] = *n;
    }
    println!("{:?}", custom_bar);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_check() {
        solve();
        assert_eq!(2 + 2, 4);
    }
}
