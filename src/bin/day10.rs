use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day10.txt").unwrap();

	let mut list = (0..=255).into_iter().collect::<Vec<_>>();
	let mut curr = 0;
	let mut skip_size = 0;
	let lengths = file.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	for length in lengths {
		let mut vec = Vec::new();
		for i in 0..length {
			vec.push(list[(i+curr) % list.len()]);
		}
		vec.reverse();
		let len = list.len();
		for i in 0..length {
			list[(i + curr) % len] = vec[i];
		}
		curr += length + skip_size;
		skip_size += 1;
	}

	let part1 = list[0] * list[1];
	println!("Day 10 part 1: {}", part1);

	let mut list = (0..=255).into_iter().collect::<Vec<_>>();
	let mut curr = 0;
	let mut skip_size = 0;
	let lengths = file.chars().map(|x| x as u32 as usize).chain([17, 31, 73, 47, 23].into_iter()).collect::<Vec<_>>();
	for _ in 0..64 {
		for &length in &lengths {
			let mut vec = Vec::new();
			for i in 0..length {
				vec.push(list[(i+curr) % list.len()]);
			}
			vec.reverse();
			let len = list.len();
			for i in 0..length {
				list[(i + curr) % len] = vec[i];
			}
			curr += length + skip_size;
			skip_size += 1;
		}
	}

	let part2 = list.chunks(16).map(|x| x.iter().fold(0, |acc, x| acc ^ x)).map(|x| format!("{:x}", x)).collect::<String>();
	println!("Day 10 part 2: {}", part2);
}