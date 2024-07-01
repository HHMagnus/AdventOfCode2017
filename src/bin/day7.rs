use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day7.txt").unwrap();

    let mut items = Vec::new();
    let mut deps = Vec::new();

    for line in file.lines() {
        if !line.contains("->") {
            continue;
        }

        let x = line.split(" -> ").collect::<Vec<_>>();
        let name = x[0].split(" ").next().unwrap();
        items.push(name);
        let mut dps = x[1].split(", ").collect::<Vec<_>>();
        deps.append(&mut dps);
    }

    let part1 = items.into_iter().find(|x| !deps.contains(x)).unwrap();
    println!("Day 7 part 1: {}", part1)
}