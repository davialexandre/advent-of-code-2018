use std::fs;
use std::io::Error;
use std::collections::HashSet;

fn main() {
    println!("Part 1 solution: {}", part_1());
    println!("Part 2 solution: {}", part_2());
}

/// Calculates the final resulting frequency by summing all the
/// frequency changes from the input file.
fn part_1() -> i32 {
    let input = get_input();

    if let Ok(v) = input {
        return v.lines().fold(0, |acc, line| acc + line.parse::<i32>().unwrap());
    }

    0
}

/// Cycles through the frequency changes in the array until finding a frequency that repeats twice
///
/// For example, using the same list of changes above, the device would loop as follows:
///
/// - Current frequency  0, change of +1; resulting frequency  1.
/// - Current frequency  1, change of -2; resulting frequency -1.
/// - Current frequency -1, change of +3; resulting frequency  2.
/// - Current frequency  2, change of +1; resulting frequency  3.
/// - (At this point, the device continues from the start of the list.)
/// - Current frequency  3, change of +1; resulting frequency  4.
/// - Current frequency  4, change of -2; resulting frequency  2, which has already been seen.
///
/// So the result will be 2
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

/// Get the contents of the input file as a String
fn get_input() -> Result<String, Error> {
    fs::read_to_string("input")
}
