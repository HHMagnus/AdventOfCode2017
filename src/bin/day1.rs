use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day1.txt").unwrap();
    let mut part1 = 0;
    let ints = file.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    let mut last = *ints.last().unwrap();
    for &i in &ints {
        if last == i {
            part1 += i;
        }
        last = i;
    }

    println!("Day 1 part 1: {}", part1);

    let mut part2 = 0;
    for i in 0..ints.len() {
        if ints[i] == ints[(i + ints.len() / 2) % ints.len()] {
            part2 += ints[i];
        }
    }
    
    println!("Day 1 part 2: {}", part2);
}