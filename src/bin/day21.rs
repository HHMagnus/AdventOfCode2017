use std::{collections::HashSet, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day21.txt").unwrap();

	let start = vec![vec!['.', '#', '.'], vec!['.', '.', '#'], vec!['#', '#', '#']];
	let patterns = file.lines().map(|x| {
		let split = x.split(" => ").collect::<Vec<_>>();
		let input = split[0].split("/").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
		let output = split[1].split("/").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
		(combinations(input), output)
	}).collect::<Vec<_>>();

	let mut curr = start;
	for _ in 0..5 {
		let s = split(curr);
		let applied = s.into_iter().map(|x| apply_pattern(x, &patterns)).collect();
		curr = concat(applied);
	}

	let part1 = curr.iter().map(|x| x.iter().filter(|&x| x == &'#').count()).sum::<usize>();
	println!("Day 21 Part 1: {}", part1);
}

fn flip(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
	input.into_iter().map(|mut x| {
		x.reverse();
		x
	}).collect()
}

fn rotate(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
	let mut vec = Vec::new();
	let maxx = input[0].len()-1;
	for x in 0..=maxx {
		let mut inner = Vec::new();
		for y in 0..input.len() {
			inner.push(input[y][maxx-x]);
		}
		vec.push(inner);
	}
	vec
}

fn split(input: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
	let split_size = if input.len() % 2 == 0 { 2 } else { 3 };
	let size = input.len() / split_size;
	let mut result = Vec::new();
	for y in 0..size {
		for x in 0..size {
			let mut vec = Vec::new();
			for vy in 0..split_size {
				let mut xvec = Vec::new();
				for vx in 0..split_size {
					let element = input[y * split_size + vy][x * split_size + vx];
					xvec.push(element);
				}
				vec.push(xvec);
			}
			result.push(vec);
		}
	}
	result
}

fn concat(input: Vec<Vec<Vec<char>>>) -> Vec<Vec<char>> {
	let mut result = Vec::new();
	if input.len() == 1 {
		return input[0].clone();
	}
	let overall_size = (input.len() as f64).sqrt() as usize;
	let split_size = input[0].len();
	for _ in 0..overall_size*split_size {
		result.push(Vec::new());
	}

	for y in 0..overall_size {
		for x in 0..overall_size {
			let curr = &input[y * overall_size + x];
			for vy in 0..curr.len() {
				for &el in &curr[vy] {
					result[y * split_size + vy].push(el);
				}
			}
		}
	}
	result
}

fn combinations(input: Vec<Vec<char>>) -> HashSet<Vec<Vec<char>>> {
	vec![
		input.clone(),
		rotate(input.clone()),
		rotate(rotate(input.clone())),
		rotate(rotate(rotate(input))),
	].into_iter().flat_map(|x| vec![x.clone(), flip(x)]).collect()
}

fn apply_pattern(input: Vec<Vec<char>>, patterns: &Vec<(HashSet<Vec<Vec<char>>>, Vec<Vec<char>>)>) -> Vec<Vec<char>> {
	for pattern in patterns {
		if pattern.0.contains(&input) {
			return pattern.1.clone();
		}
	}
	unreachable!("No known pattern");
}