use std::{collections::HashMap, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day6.txt").unwrap();
    let mut nums = file.split("\t").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let mut states = HashMap::new();

    let mut redistribution_cycles = 0;

    loop {
        if states.contains_key(&nums) {
            println!("Day 6 part 1: {}", redistribution_cycles);
            println!("Day 6 part 2: {}", redistribution_cycles - states[&nums]);
            break;
        }
        states.insert(nums.clone(), redistribution_cycles);
        redistribution_cycles += 1;

        let mut idx = nums.iter().enumerate().max_by_key(|&(i, &x)| (x, nums.len()-i)).unwrap().0;

        let mut num = nums[idx];
        nums[idx] = 0;
        idx += 1;
        let len = nums.len();

        while num > 0 {
            num -= 1;
            nums[idx % len] += 1;
            idx += 1;
        }
    }
}