use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day5.txt").unwrap();
    let mut nums = file.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut i = 0;
    let mut part1 = 0;
    while i >= 0 && i < nums.len() as i32 {
        part1 += 1;
        let num = nums[i as usize];
        nums[i as usize] = num+1;
        i += num;
    }
    println!("Day 5 part 1: {}", part1);
    
    let mut nums = file.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut i = 0;
    let mut part2 = 0;
    while i >= 0 && i < nums.len() as i32 {
        part2 += 1;
        let num = nums[i as usize];
        nums[i as usize] = num + if num >= 3 { -1 } else { 1 };
        i += num;
    }
    println!("Day 5 part 2: {}", part2);
}