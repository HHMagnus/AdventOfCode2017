use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day13.txt").unwrap();

	let input = file.lines().map(|x| { let s = x.split(": ").collect::<Vec<_>>(); (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap())}).collect::<Vec<_>>();

	let mut scanners = input.iter().map(|_| (0, 1)).collect::<Vec<_>>();
	
	let mut part1 = 0;
	for depth in 0..=*input.iter().map(|(x, _)| x).max().unwrap() {
		if let Some((j, x)) = input.iter().enumerate().find(|(_, &(x, _))| x == depth) {
			if scanners[j].0 == 0 {
				part1 += x.0*x.1;
			}
		}

		for i in 0..input.len() {
			scanners[i].0 += scanners[i].1;
			if scanners[i].0 == 0 {
				scanners[i].1 *= -1;
			}
			if scanners[i].0 == input[i].1-1 {
				scanners[i].1 *= -1;
			}
		}
	}

	println!("Day 13 part 1: {}", part1);

	let mut part2 = 0;

	while input.iter().any(|x| (x.0 + part2) % ((x.1 - 1) * 2) == 0) {
		part2 += 1;
	}

	println!("Day 13 part 2: {}", part2);
}