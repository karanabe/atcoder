use proconio::input;

fn main() {
    input! {
        sab: char,
        sac: char,
        sbc: char,
    }

    let middle = if sab == '<' && sac == '<' {
        if sbc == '<' { // B < C
            "B"
        } else {
            "C"
        }
    } else if sab == '>' && sac == '>' {
        if sbc == '<' {
            "C"
        } else {
            "B"
        }
    } else if sab == '<' && sac == '>' {
        "A"
    } else if sab == '>' && sac == '<' {
        "A"
    } else {
        if sbc == '<' {
            "B"
        } else {
            "C"
        }
    };

    println!("{}", middle);
}
