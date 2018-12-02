use std::fs;

fn main() {
    println!("Part 1 solution: {}", part_1());
}

fn part_1() -> i32 {
    let input = fs::read_to_string("input");

    if let Ok(v) = input {
        return v.lines().fold(0, |acc, line| acc + line.parse::<i32>().unwrap());
    }

    0
}
