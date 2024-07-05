use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day25.txt").unwrap();

	let split = file.split("\n\n").collect::<Vec<_>>();

	let start = split[0].lines().next().unwrap().replace("Begin in state ", "").chars().next().unwrap();
	let steps = split[0].lines().next_back().unwrap().replace("Perform a diagnostic checksum after ", "").replace(" steps.", "").parse::<i32>().unwrap();

	let map = split[1..].into_iter().map(|x| {
		let lines = x.lines().collect::<Vec<_>>();
		let state = lines[0].replace("In state ", "").chars().next().unwrap();
		let zv = lines[2].contains("1");
		let zdir = lines[3].contains("right");
		let zn = lines[4].replace("    - Continue with state ", "").chars().next().unwrap();
		let ov = lines[6].contains("1");
		let odir = lines[7].contains("right");
		let on = lines[8].replace("    - Continue with state ", "").chars().next().unwrap();
		(state, (zv, zdir, zn, ov, odir, on))
	}).collect::<HashMap<_,_>>();

	let mut set = HashSet::new();
	let mut pos = 0;
	let mut curr = start;

	for _ in 0..steps {
		let m = map[&curr];
		if set.contains(&pos) {
			curr = m.5;
			if !m.3 {
				set.remove(&pos);
			}
			pos += if m.4 { 1 } else { -1 };
		} else {
			curr = m.2;
			if m.0 {
				set.insert(pos);
			}
			pos += if m.1 { 1 } else { -1 };
		}
	}

	println!("Day 25 part 1: {}", set.len());
}