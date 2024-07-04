use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day20.txt").unwrap();

	let particles = file.lines().map(|line| {
		let split = line.split(", ").collect::<Vec<_>>();
		let p = split[0].replace("p=<", "").replace(">", "").split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let v = split[1].replace("v=<", "").replace(">", "").split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let a = split[2].replace("a=<", "").replace(">", "").split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

		((p[0], p[1], p[2]), (v[0], v[1], v[2]), (a[0], a[1], a[2]))
	}).collect::<Vec<_>>();

	let mut part1particles = particles.clone();
	
	for _ in 0..1000 {
		part1particles = part1particles.into_iter().map(|(p, (v1, v2, v3), (a1, a2, a3))| (p, (v1+a1, v2+a2, v3+a3), (a1, a2, a3))).collect();
		part1particles = part1particles.into_iter().map(|((p1, p2, p3), (v1, v2, v3), a)| ((p1+v1, p2+v2, p3+v3), (v1, v2, v3), a)).collect();
	}

	let part1 = part1particles.iter().enumerate().min_by_key(|(_, ((x, y, z), _, _))| x.abs() + y.abs() + z.abs()).unwrap().0;
	println!("Day 20 part 1: {}", part1);

	let mut part2particles = particles;
	
	for _ in 0..1000 {
		part2particles = part2particles.into_iter().map(|(p, (v1, v2, v3), (a1, a2, a3))| (p, (v1+a1, v2+a2, v3+a3), (a1, a2, a3))).collect();
		part2particles = part2particles.into_iter().map(|((p1, p2, p3), (v1, v2, v3), a)| ((p1+v1, p2+v2, p3+v3), (v1, v2, v3), a)).collect();
		
		part2particles = part2particles.iter().filter(|(p, _, _)| part2particles.iter().filter(|(p0, _, _)| p0 == p).count() == 1).cloned().collect();
	}

	println!("Day 20 part 2: {}", part2particles.len());
}