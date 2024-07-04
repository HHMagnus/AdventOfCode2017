use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq)]
enum Dir {
	Down,
	Up,
	Right,
	Left
}

fn main() {
	let file = read_to_string("input/day19.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let start = map.iter().filter(|(&(x, y), _)| {
		let neighs = [
			(x, y - 1),
			(x, y + 1),
			(x - 1, y),
			(x + 1, y),
		].into_iter().filter(|xy| map.contains_key(xy) && map.get(xy).unwrap() != &' ').count();
		map[&(x, y)] == '|' && neighs == 1
	}).next().map(|x| *x.0).unwrap();

	let mut curr = start;
	let mut dir = Dir::Down;

	let mut vec = Vec::new();
	let mut steps = 0;

	loop {
		steps += 1;

		let (x, y) = curr;
		curr = match dir {
			Dir::Down => (x, y + 1),
			Dir::Up => (x, y - 1),
			Dir::Right => (x + 1, y),
			Dir::Left => (x - 1, y),
		};

		let c = map[&curr];

		if c == ' ' {
			break;
		}

		if c == '+' {
			if dir == Dir::Down || dir == Dir::Up {
				if map[&(curr.0 + 1, curr.1)] == '-' {
					dir = Dir::Right;
				} else {
					dir = Dir::Left;
				}
			} else {
				if map[&(curr.0, curr.1 - 1)] == '|' {
					dir = Dir::Up;
				} else {
					dir = Dir::Down;
				}
			}
		}
		
		if c.is_uppercase() {
			vec.push(c);
		}
	}

	let part1 = vec.into_iter().collect::<String>();
	println!("Day 19 part 1: {}", part1);
	println!("Day 19 part 2: {}", steps);
}