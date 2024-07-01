use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day16.txt").unwrap();

	let mut vec = (0..16).map(|x| char::from_u32('a' as u32 + x).unwrap()).collect::<Vec<_>>();

	for line in file.split(",") {
		if line.starts_with("s") {
			let xs = line.replace("s", "").parse::<usize>().unwrap();
			let mut new = Vec::new();
			for i in vec.len()-xs..vec.len() {
				new.push(vec[i]);
			}
			for i in 0..vec.len()-xs {
				new.push(vec[i]);
			}
			vec = new;
		}
		if line.starts_with("x") {
			let xs = line[1..].split("/").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
			let temp = vec[xs[0]];
			vec[xs[0]] = vec[xs[1]];
			vec[xs[1]] = temp;
		}
		if line.starts_with("p") {
			let xs = line[1..].split("/").map(|x| x.chars().next().unwrap()).collect::<Vec<_>>();
			let a = vec.iter().enumerate().find(|x| x.1 == &xs[0]).unwrap().0;
			let b = vec.iter().enumerate().find(|x| x.1 == &xs[1]).unwrap().0;
			let temp = vec[a];
			vec[a] = vec[b];
			vec[b] = temp;
		}
	}

	let part1 = vec.into_iter().collect::<String>();
	println!("Day 16 part 1: {}", part1);
}