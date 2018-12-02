use std::fs;
use std::io::Error;
use std::collections::HashSet;

fn main() {
    println!("Part 1 solution: {}", part_1());
    println!("Part 2 solution: {}", part_2());
}

fn part_1() -> i32 {
    let input = get_input();

    if let Ok(v) = input {
        return v.lines().fold(0, |acc, line| acc + line.parse::<i32>().unwrap());
    }

    0
}

fn part_2() -> i32 {
    let input = get_input();

    let mut frequencies = HashSet::new();

    let mut current_frequency: i32 = 0;

    frequencies.insert(current_frequency);

    if let Ok(v) = input {
        loop {
            for line in v.lines() {
                current_frequency = current_frequency + line.parse::<i32>().unwrap();
                if !frequencies.insert(current_frequency) {
                    return current_frequency;
                }
            }
        }
    }

    0
}

fn get_input() -> Result<String, Error> {
    fs::read_to_string("input")
}
