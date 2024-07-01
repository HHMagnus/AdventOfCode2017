use std::{collections::{HashSet, VecDeque}, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day14.txt").unwrap();
	
	let mut part1 = 0;

	let mut used = HashSet::new();

	for y in 0..128 {
		let str = format!("{}-{}", file, y);
		let hash = knot_hash(&str);

		let mut xs = 0;

		for mut h in hash {
			for i in 0..8 {
				if h % 2 == 1 {
					part1 += 1;
					used.insert((xs * 8 + 8 - i, y));
				}
				h >>= 1;
			}
			xs += 1;
		}
	}
	
	println!("Day 14 part 1: {}", part1);

	let mut visited = HashSet::new();
	let mut part2 = 0;

	while let Some(&start) = used.iter().find(|&x| !visited.contains(x)) {
		let mut queue = VecDeque::new();
		queue.push_back(start);
		visited.insert(start);

		while let Some(next) = queue.pop_front() {
			let neighbors = [
				(next.0 - 1, next.1),
				(next.0 + 1, next.1),
				(next.0, next.1 - 1),
				(next.0, next.1 + 1),
			];
			for n in neighbors {
				if !used.contains(&n) {
					continue;
				}
				if visited.contains(&n) {
					continue;
				}
				visited.insert(n);
				queue.push_back(n);
			}
		}

		part2 += 1;
	}

	println!("Day 14 part 2: {}", part2);
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