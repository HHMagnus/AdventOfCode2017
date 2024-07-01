use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day9.txt").unwrap();

    let chars = file.chars().collect::<Vec<_>>();
    let mut score = 1;
    let mut part1 = 0;
    let mut garbage = false;

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if garbage {
            if c == '>' {
                garbage = false;
            }
            if c == '!' {
                i += 1;
            }
            i += 1;
            continue;
        }
        if c == '{' {
            part1 += score;
            score += 1;
        }
        if c == '}' && score > 1 {
            score -= 1;
        }
        i += 1;
    }

    println!("Day 9 part 1: {}", part1);
}