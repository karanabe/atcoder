use proconio::input;

fn main() {
    input! {
        n: usize,
        healths: [i64; n],
    }

    let mut max_turns: i64 = 0;
    for &health in &healths {
        let cur_damage = health / 5;
        max_turns += cur_damage * 3;
        let mut sub_health = health - cur_damage*5;

        while sub_health > 0 {
            max_turns += 1;

            if max_turns % 3 == 0 {
                sub_health -= 3;
            } else {
                sub_health -= 1;
            }
        }
    }

    println!("{}", max_turns);
}
