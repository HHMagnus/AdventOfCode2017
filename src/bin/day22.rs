use std::{collections::HashSet, fs::read_to_string};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
	let file = read_to_string("input/day22.txt").unwrap();

    let mut map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter_map(move |(x, c)| if c == '#' { Some((x as i32, y as i32)) } else { None })).collect::<HashSet<_>>();

    let mut curr = (file.lines().next().unwrap().len() as i32 / 2, file.lines().count() as i32 / 2);
    let mut dir = Direction::Up;

    let mut burst = 0;

    for _ in 0..10000 {
        if map.contains(&curr) {
            dir = right(dir);
            map.remove(&curr);
        } else {
            dir = left(dir);
            map.insert(curr);
            burst += 1;
        }

        let (x, y) = curr;
        curr = match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
    }

    println!("Day 22 part 1: {}", burst);
    
    let mut infected = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter_map(move |(x, c)| if c == '#' { Some((x as i32, y as i32)) } else { None })).collect::<HashSet<_>>();
    let mut weaken = HashSet::new();
    let mut flagged = HashSet::new();

    let mut curr = (file.lines().next().unwrap().len() as i32 / 2, file.lines().count() as i32 / 2);
    let mut dir = Direction::Up;

    let mut burst = 0;

    for _ in 0..10000000 {
        if infected.contains(&curr) {
            dir = right(dir);
            infected.remove(&curr);
            flagged.insert(curr);
        } else if weaken.contains(&curr) {
            weaken.remove(&curr);
            infected.insert(curr);
            burst += 1;
        } else if flagged.contains(&curr) {
            dir = reverse(dir);
            flagged.remove(&curr);
        } else {
            dir = left(dir);
            weaken.insert(curr);
        }

        let (x, y) = curr;
        curr = match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
    }

    println!("Day 22 part 2: {}", burst);
}

fn right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn reverse(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}