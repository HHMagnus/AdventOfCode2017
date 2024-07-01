use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day11.txt").unwrap();

	let mut q: i32 = 0;
	let mut r: i32 = 0;
	let mut s: i32 = 0;
	let mut part2 = 0;
	for x in file.split(",") {
		match x {
			"s" => {
				q -= 1;
				s += 1;
			},
			"n" => {
				q += 1;
				s -= 1;
			},
			"ne" => {
				s -= 1;
				r += 1;
			},
			"se" => {
				q -= 1;
				r += 1;
			},
			"sw" => {
				s += 1;
				r -= 1;
			},
			"nw" => {
				q += 1;
				r -= 1;
			},
			x => unreachable!("Unknown {}", x),
		}
		part2 = part2.max(q.abs().max(r.abs()).max(s.abs()));
	}

	let part1 = q.abs().max(r.abs()).max(s.abs());
	println!("Day 11 part 1: {}", part1);
	println!("Day 11 part 2: {}", part2);
}