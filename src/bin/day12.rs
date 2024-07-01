use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day12.txt").unwrap();
	let mut states = HashMap::new();

	for line in file.lines() {
		let s = line.split(" <-> ").collect::<Vec<_>>();
		let vec = s[1].split(", ").collect::<Vec<_>>();
		states.insert(s[0], vec);
	}

	let mut visited = Vec::new();
	let mut queue = VecDeque::new();
	queue.push_back("0");
	visited.push("0");

	while let Some(next) = queue.pop_front() {
		let vec = states.get(next).unwrap();
		for v in vec {
			if visited.contains(v) { continue; }
			visited.push(v);
			queue.push_back(v);
		}
	}

	let part1 = visited.len();
	println!("Day 12 part 1: {}", part1);
	
	let mut part2 = 0;

	let mut visited = HashSet::new();
	while let Some((&start, _)) = states.iter().find(|x| !visited.contains(x.0)) {
		part2 += 1;
		let mut queue = VecDeque::new();
		queue.push_back(start);

		while let Some(next) = queue.pop_front() {
			let vec = states.get(next).unwrap();
			for v in vec {
				if visited.contains(v) { continue; }
				visited.insert(v);
				queue.push_back(v);
			}
		}
	}

	println!("Day 12 part 2: {}", part2);
}