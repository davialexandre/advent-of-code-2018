use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Part 1 solution: {}", part_1());
}

/// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number
/// that have an ID containing exactly two of any letter and then separately counting those with
/// exactly three of any letter. You can multiply those two counts together to get a rudimentary
/// checksum and compare it to what your device predicts.
///
/// For example, if you see the following box IDs:
///
///  - abcdef contains no letters that appear exactly two or three times.
///  - bababc contains two a and three b, so it counts for both.
///  - abbcde contains two b, but no letter appears exactly three times.
///  - abcccd contains three c, but no letter appears exactly two times.
///  - aabcdd contains two a and two d, but it only counts once.
///  - abcdee contains two e.
///  - ababab contains three a and three b, but it only counts once.
///
/// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them
/// contain a letter which appears exactly three times. Multiplying these together produces a
/// checksum of 4 * 3 = 12.
fn part_1() -> i32 {
    let input = get_input();

    let mut boxes_with_2 = 0;
    let mut boxes_with_3 = 0;

    for line in input.lines() {
        let mut letters_count: HashMap<char, u32> = HashMap::new();

        for letter in line.chars() {
            let count = letters_count.entry(letter).or_insert(0);
            *count += 1
        }

        if letters_count.values().any(|&v| v == 2) {
            boxes_with_2 += 1
        }

        if letters_count.values().any(|&v| v == 3) {
            boxes_with_3 += 1
        }

    }

    boxes_with_2 * boxes_with_3
}

/// Get the contents of the input file as a String
fn get_input() -> String {
    match fs::read_to_string("input") {
        Ok(v) => return v,
        _ => String::new()
    }
}
