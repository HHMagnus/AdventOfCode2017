use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day7.txt").unwrap();

    let mut items = Vec::new();
    let mut all_deps = Vec::new();
    let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in file.lines() {
        let x = line.split(" -> ").collect::<Vec<_>>();
        let spl = x[0].split(" ").collect::<Vec<_>>();
        let name = spl[0];
        let weight = spl[1].replace("(", "").replace(")", "").parse::<i32>().unwrap();
        items.push((name, weight));

        if !line.contains("->") {
            continue;
        }
        let mut dps = x[1].split(", ").collect::<Vec<_>>();
        deps.insert(name, dps.clone());
        all_deps.append(&mut dps);
    }

    let part1 = items.iter().find(|(x,_)| !all_deps.contains(x)).unwrap().0;
    println!("Day 7 part 1: {}", part1);

    let part2 = part2(part1, &items, &deps).1;
    println!("Day 7 part 2: {}", part2);
}

fn part2(item: &str, items: &Vec<(&str, i32)>, deps: &HashMap<&str, Vec<&str>>) -> (bool, i32) {
    let weight = items.iter().find(|&&(x, _)| x == item).unwrap().1;
    if !deps.contains_key(item) {
        return (false, weight);
    }
    let dps = deps.get(item).unwrap();
    let weights = dps.iter().map(|&x| (x, part2(&x, items, deps))).collect::<Vec<_>>();
    if let Some(&(_,re)) = weights.iter().find(|(_, (x, _))| *x) {
        return re;
    }
    let weights = weights.into_iter().map(|x| (x.0, x.1.1)).collect::<Vec<_>>();
    let set = weights.iter().map(|(_, x)| x).collect::<HashSet<_>>();
    if set.len() == 1 {
        return (false, weights.iter().map(|(_, x)| x).sum::<i32>() + weight);
    }
    for &x in set {
        if weights.iter().filter(|&&(_, y)| x == y).count() == 1 {
            let other =  *weights.iter().find(|&&(_, y)| x != y).unwrap();
            let curr = *weights.iter().find(|&&(_, y)| x == y).unwrap();
            let diff = other.1 - curr.1;
            let weight = items.iter().find(|x| x.0 == curr.0).unwrap().1;
            return (true, weight + diff);
        }
    }
    (false, 0)
}