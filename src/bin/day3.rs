use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day3.txt").unwrap();
    let input = file.parse::<i64>().unwrap();

    let coords = part1(input);
    let part1 = coords.0.abs() + coords.1.abs();
    println!("Day 3 part 1: {}", part1);
}

fn part1(input: i64) -> (i64, i64) {
    let mut miny = 0;
    let mut maxy = 0;
    let mut minx = 0;
    let mut maxx = 0;

    let mut curr = (0,0);
    let mut steps = 1;
    loop {
        while curr.0 <= maxx {
            steps += 1;
            curr = (curr.0 + 1, curr.1);

            if steps == input {
                return curr;
            }
        }
        maxx = curr.0;
        while curr.1 >= miny {
            steps += 1;
            curr = (curr.0, curr.1 - 1);

            if steps == input {
                return curr;
            }
        }
        miny = curr.1;
        while curr.0 >= minx {
            steps += 1;
            curr = (curr.0 - 1, curr.1);

            if steps == input {
                return curr;
            }
        }
        minx = curr.0;
        while curr.1 <= maxy {
            steps += 1;
            curr = (curr.0, curr.1 + 1);

            if steps == input {
                return curr;
            }
        }
        maxy = curr.1;
    }
}