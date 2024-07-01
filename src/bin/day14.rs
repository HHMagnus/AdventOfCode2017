use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day14.txt").unwrap();
	
	let mut part1 = 0;

	for i in 0..128 {
		let str = format!("{}-{}", file, i);
		let hash = knot_hash(&str);

		for mut x in hash {
			while x > 0 {
				if x % 2 == 1 {
					part1 += 1;
				}
				x >>= 1;
			}
		}
	}

	println!("Day 14 part 1: {}", part1);
}

fn knot_hash(hash: &str) -> Vec<i32> {
	let mut list = (0..=255).into_iter().collect::<Vec<_>>();
	let mut curr = 0;
	let mut skip_size: usize = 0;
	let lengths = hash.chars().map(|x| x as u32 as usize).chain([17, 31, 73, 47, 23].into_iter()).collect::<Vec<_>>();
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

	let part2 = list.chunks(16).map(|x| x.iter().fold(0, |acc, x| acc ^ x)).collect::<Vec<_>>();
	part2
}