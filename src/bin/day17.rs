use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day17.txt").unwrap();
	let input = file.parse::<usize>().unwrap();

	let mut chain = vec![(0, 0)];

	let mut curr = 0;
	for i in 1..2018 {
		for _ in 0..input {
			curr = chain[curr].1;
		}
		let next = chain[curr].1;
		chain[curr].1 = chain.len();
		curr = chain.len();
		chain.push((i, next));
	}

	let part1 = chain[chain.iter().find(|x| x.0 == 2017).unwrap().1].0;
	println!("Day 17 part 1: {}", part1);
}