use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day15.txt").unwrap();
	let lines = file.lines().collect::<Vec<_>>();
	let ga = lines[0].replace("Generator A starts with ", "").parse::<u128>().unwrap();
	let gb = lines[1].replace("Generator B starts with ", "").parse::<u128>().unwrap();

	let af = 16807;
	let bf = 48271;

	let mut lasta = ga;
	let mut lastb = gb;

	let mut part1 = 0;

	for _ in 0..40000000 {
		lasta *= af;
		lasta %= 2147483647;
		lastb *= bf;
		lastb %= 2147483647;

		if lasta & (u16::MAX as u128) == lastb & (u16::MAX as u128) {
			part1 += 1;
		}
	}

	println!("Day 15 part 1: {}", part1);

	let mut lasta = ga;
	let mut lastb = gb;

	let mut part2 = 0;

	for _ in 0..5000000 {
		loop {
			lasta *= af;
			lasta %= 2147483647;
			if lasta % 4 == 0 {
				break;
			}
		}
		loop {
			lastb *= bf;
			lastb %= 2147483647;
			if lastb % 8 == 0 {
				break;
			}
		}

		if lasta & (u16::MAX as u128) == lastb & (u16::MAX as u128) {
			part2 += 1;
		}
	}

	println!("Day 15 part 2: {}", part2);
}