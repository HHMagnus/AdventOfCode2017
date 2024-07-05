use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
enum Val {
    Register(char),
    Value(i64),
}

#[derive(Debug)]
enum Instruction {
    Set(char, Val),
    Sub(char, Val),
    Mul(char, Val),
    Jnz(Val, Val),
}

fn main() {
	let file = read_to_string("input/day23.txt").unwrap();

    let instructions = file.lines().map(|line| {
        let xs = line.split(" ").collect::<Vec<_>>();
        match xs[0] {
            "set" => {
                let x = xs[1];
                let y = xs[2];
                let val = match y.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(y.chars().next().unwrap()),
                };
                Instruction::Set(x.chars().next().unwrap(), val)
            },
            "sub" => {
                let x = xs[1];
                let y = xs[2];
                let val = match y.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(y.chars().next().unwrap()),
                };
                Instruction::Sub(x.chars().next().unwrap(), val)
            },
            "mul" => {
                let x = xs[1];
                let y = xs[2];
                let val = match y.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(y.chars().next().unwrap()),
                };
                Instruction::Mul(x.chars().next().unwrap(), val)
            },
            "jnz" => {
                let x = xs[1];
                let val1 = match x.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(x.chars().next().unwrap()),
                };
                let y = xs[2];
                let val2 = match y.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(y.chars().next().unwrap()),
                };
                Instruction::Jnz(val1, val2)
            },
            x => unreachable!("unexpected: {}", x),
        }
    }).collect::<Vec<_>>();

    part1(&instructions);
    
    let part2range = find_range_for_part2(&instructions);
    let mut part2 = 0;
    
    for x in (part2range.0..=part2range.1).step_by(17) {
        if !is_prime(x) {
            part2 += 1;
        }
    }

    println!("Day 23 part 2: {}", part2);
}

fn is_prime(n: i64) -> bool {
    let limit = (n as f64).sqrt() as i64;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn part1(instructions: &Vec<Instruction>) {
    let mut registers = HashMap::new();
    let mut i = 0;

    let mut part1 = 0;

    while i >= 0 && i < instructions.len() as i64 {
        let instruction = &instructions[i as usize];
        match instruction {
            Instruction::Set(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x = val;
            },
            Instruction::Sub(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x -= val;
            },
            Instruction::Mul(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x *= val;
                part1 += 1;
            },
            Instruction::Jnz(x, y) => {
                let val1 = next(x, &registers);
                let val2 = next(y, &registers);
                if val1 != 0 {
                    i += val2;
                    continue;
                }
            },
        }

        i += 1;
    }

    println!("Day 23 part 1: {}", part1);
}

fn find_range_for_part2(instructions: &Vec<Instruction>) -> (i64, i64) {
    let mut registers = HashMap::new();
    registers.insert('a', 1);
    let mut i = 0;
    let mut count = 0;

    while count < 1000 {
        count += 1;
        let instruction = &instructions[i as usize];
        match instruction {
            Instruction::Set(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x = val;
            },
            Instruction::Sub(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x -= val;
            },
            Instruction::Mul(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x *= val;
            },
            Instruction::Jnz(x, y) => {
                let val1 = next(x, &registers);
                let val2 = next(y, &registers);
                if val1 != 0 {
                    i += val2;
                    continue;
                }
            },
        }

        i += 1;
    }
    
    return (registers[&'b'], registers[&'c'])
}

fn next(val: &Val, registers: &HashMap<char, i64>) -> i64 {
    match val {
        Val::Register(x) => *registers.get(x).unwrap_or(&0),
        Val::Value(x) => *x,
    }
}