use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day8.txt").unwrap();

    let mut state = HashMap::new();
    let mut part2 = 0;

    for line in file.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
        let val = line[4];
        let op = line[5];
        let i = line[6].parse::<i32>().unwrap();
        let entry = *state.entry(val).or_insert(0);
        let condition = match op {
            "<" => entry < i,
            ">" => entry > i,
            "!=" => entry != i,
            "==" => entry == i,
            ">=" => entry >= i,
            "<=" => entry <= i,
            x => unreachable!("Unknown op={}", x),
        };

        if condition {
            let name = line[0];
            let op = line[1];
            let val = line[2].parse::<i32>().unwrap();
            let entry = state.entry(name).or_insert(0);
            *entry += match op {
                "inc" => val,
                "dec" => -val,
                x => unreachable!("Unknown action={}", x),
            };
            part2 = part2.max(*entry);
        }
    }

    let part1 = state.values().max().unwrap();
    println!("Day 8 part 1: {}", part1);
    println!("Day 8 part 2: {}", part2);
}