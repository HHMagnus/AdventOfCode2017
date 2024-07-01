use std::{collections::HashSet, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day4.txt").unwrap();

    let part1 = file.lines().filter(|x| {
        let s = x.split(" ").collect::<Vec<_>>();
        let h = s.iter().collect::<HashSet<_>>();
        s.len() == h.len()
    }).count();

    println!("Day 4 part 1: {}", part1);

    let part2 = file.lines().filter(|x| {
        let s = x.split(" ").collect::<Vec<_>>();
        let h = s.iter().collect::<HashSet<_>>();
        let f = s.len() == h.len();
        let sn = s.clone().into_iter().map(|y| {
            let mut chars = y.chars().collect::<Vec<_>>();
            chars.sort();
            chars.into_iter().collect::<String>()
        }).collect::<HashSet<_>>();
        f && sn.len() == s.len()
    }).count();

    println!("Day 4 part 2: {}", part2);
}