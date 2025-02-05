use std::{collections::{HashMap, VecDeque}, fs::read_to_string};

#[derive(Debug)]
enum Val {
    Register(char),
    Value(i64),
}

#[derive(Debug)]
enum Instruction {
    Snd(Val),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(char),
    Jgz(Val, Val),
}

fn main() {
	let file = read_to_string("input/day18.txt").unwrap();

    let instructions = file.lines().map(|line| {
        let xs = line.split(" ").collect::<Vec<_>>();
        match xs[0] {
            "snd" => {
                let x = xs[1];
                let val = match x.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(x.chars().next().unwrap()),
                };
                Instruction::Snd(val)
            },
            "set" => {
                let x = xs[1];
                let y = xs[2];
                let val = match y.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(y.chars().next().unwrap()),
                };
                Instruction::Set(x.chars().next().unwrap(), val)
            },
            "add" => {
                let x = xs[1];
                let y = xs[2];
                let val = match y.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(y.chars().next().unwrap()),
                };
                Instruction::Add(x.chars().next().unwrap(), val)
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
            "mod" => {
                let x = xs[1];
                let y = xs[2];
                let val = match y.parse::<i64>() {
                    Ok(x) => Val::Value(x),
                    Err(_) => Val::Register(y.chars().next().unwrap()),
                };
                Instruction::Mod(x.chars().next().unwrap(), val)
            },
            "rcv" => {
                let x = xs[1];
                Instruction::Rcv(x.chars().next().unwrap())
            },
            "jgz" => {
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
                Instruction::Jgz(val1, val2)
            },
            x => unreachable!("unexpected: {}", x),
        }
    }).collect::<Vec<_>>();

    let part1 = part1(&instructions);
    println!("Day 18 part 1: {}", part1);

    let mut registers1 = HashMap::new();
    registers1.insert('p', 0);
    let mut registers2 = HashMap::new();
    registers2.insert('p', 1);
    let mut recv1 = VecDeque::new();
    let mut recv2 = VecDeque::new();
    let mut i1 = 0;
    let mut i2 = 0;

    let mut part2 = 0;

    i1 = part2run(&instructions, &mut recv2, i1, &mut registers1, &mut recv1);

    while !recv1.is_empty() || !recv2.is_empty() {
        if recv1.len() > 0 {
            i2 = part2run(&instructions, &mut recv1, i2, &mut registers2, &mut recv2);
            part2 += recv2.len();
        }
        if recv2.len() > 0 {
            i1 = part2run(&instructions, &mut recv2, i1, &mut registers1, &mut recv1);
        }
    }

    println!("Day 18 part 2: {}", part2);
}

fn next(val: &Val, registers: &HashMap<char, i64>) -> i64 {
    match val {
        Val::Register(x) => *registers.get(x).unwrap_or(&0),
        Val::Value(x) => *x,
    }
}

fn part1(instructions: &Vec<Instruction>) -> i64 {
    let mut last = 0;
    let mut registers = HashMap::new();

    let mut i = 0i64;
    while i >= 0 && i < instructions.len() as i64 {
        let instruction = &instructions[i as usize];
        match instruction {
            Instruction::Snd(x) => {
                last = next(x, &registers);
            },
            Instruction::Set(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x = val;
            },
            Instruction::Add(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x += val;
            },
            Instruction::Mul(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x *= val;
            },
            Instruction::Mod(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                if val != 0 {
                    *register_x %= val;
                }
            },
            Instruction::Rcv(x) => {
                let val = next(&Val::Register(*x), &registers);
                if val != 0 {
                    return last;
                }
            },
            Instruction::Jgz(x, y) => {
                let val1 = next(x, &registers);
                let val2 = next(y, &registers);
                if val1 > 0 {
                    i += val2;
                    continue;
                }
            },
        }

        i += 1;
    }

    unreachable!("No part 1 solution");
}

fn part2run(instructions: &Vec<Instruction>, receiving: &mut VecDeque<i64>, mut i: i64, registers: &mut HashMap<char, i64>, output: &mut VecDeque<i64>) -> i64 {
    while i >= 0 && i < instructions.len() as i64 {
        let instruction = &instructions[i as usize];
        match instruction {
            Instruction::Snd(x) => {
                let send = next(x, &registers);
                output.push_back(send);
            },
            Instruction::Set(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x = val;
            },
            Instruction::Add(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x += val;
            },
            Instruction::Mul(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                *register_x *= val;
            },
            Instruction::Mod(x, y) => {
                let val = next(y, &registers);
                let register_x = registers.entry(*x).or_insert(0);
                if val != 0 {
                    *register_x %= val;
                }
            },
            Instruction::Rcv(x) => {
                if receiving.is_empty() {
                    return i;
                }
                let register_x = registers.entry(*x).or_insert(0);
                *register_x = receiving.pop_front().unwrap();
            },
            Instruction::Jgz(x, y) => {
                let val1 = next(x, &registers);
                let val2 = next(y, &registers);
                if val1 > 0 {
                    i += val2;
                    continue;
                }
            },
        }

        i += 1;
    }

    unreachable!("No part 1 solution");
}