use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day2.txt").unwrap();

    let part1 = file.lines().map(|x| {
        let cs = x.split("\t").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        cs.iter().max().unwrap() - cs.iter().min().unwrap()
    }).sum::<u32>();
    
    println!("Day 2 part 1: {}", part1);
    
    let part2 = file.lines().map(|x| {
        let cs = x.split("\t").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        for i in 0..cs.len() {
            for j in i+1..cs.len() {
                let max = cs[i].max(cs[j]);
                let min = cs[i].min(cs[j]);
                if max % min == 0 {
                    return max / min;
                }
            }
        }
        unreachable!("Not found for line: {}", x);
    }).sum::<u32>();
    
    println!("Day 2 part 2: {}", part2);
}