use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day24.txt").unwrap();
	let input = file.lines().map(|x| {
		let split = x.split("/").collect::<Vec<_>>();
		(split[0].parse::<usize>().unwrap(), split[1].parse::<usize>().unwrap())
	}).collect::<Vec<_>>();

	let part1 = part1(0, 0, Vec::new(), &input);
	println!("Day 24 part 1: {}", part1);

	let part2 = part2(0, (0, 0), Vec::new(), &input).1;
	println!("Day 24 part 2: {}", part2);
}

fn part1(curr: usize, score: usize, taken: Vec<usize>, input: &Vec<(usize, usize)>) -> usize {
	let nexts = input.iter().enumerate().filter(|(x, _)| !taken.contains(x)).filter(|(_, x)| x.0 == curr || x.1 == curr).collect::<Vec<_>>();
	nexts.into_iter().map(|x| {
		let mut taken = taken.clone();
		taken.push(x.0);
		let curr = if x.1.0 == curr { x.1.1 } else { x.1.0 };
		part1(curr, score + x.1.0 + x.1.1, taken, input)
	}).max().unwrap_or(score)
}

fn part2(curr: usize, score: (usize, usize), taken: Vec<usize>, input: &Vec<(usize, usize)>) -> (usize, usize) {
	let nexts = input.iter().enumerate().filter(|(x, _)| !taken.contains(x)).filter(|(_, x)| x.0 == curr || x.1 == curr).collect::<Vec<_>>();
	nexts.into_iter().map(|x| {
		let mut taken = taken.clone();
		taken.push(x.0);
		let curr = if x.1.0 == curr { x.1.1 } else { x.1.0 };
		part2(curr, (score.0 + 1, score.1 + x.1.0 + x.1.1), taken, input)
	}).max().unwrap_or(score)
}