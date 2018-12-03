use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Part 1 solution: {}", part_1());
    println!("Part 2 solution: {}", part_2());
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

/// Finds the only two boxes wich the diff between their IDs will be 1
///
/// The boxes will have IDs which differ by exactly one character at the same position in both
/// strings. For example, given the following box IDs:
///
/// abcde
/// fghij
/// klmno
/// pqrst
/// fguij
/// axcye
/// wvxyz
///
/// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).
/// However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those
/// must be the correct boxes.
fn part_2() -> String {
    let input = get_input();

    let mut labels: Vec<String> = Vec::new();
    for line in input.lines() {
        labels.push(String::from(line))
    }

    labels.sort();
    let mut i = 0;
    while i < labels.len() -1 {
        match diff_words(&labels[i], &labels[i + 1]) {
            StringDiff { difference: 1, equal } => return equal,
            _ => (),
        }
        i += 1
    }

    String::new()
}

/// Compares the two Strings and returns a StringDiff struct representing the diff between them
fn diff_words(str1: &str, str2: &str) -> StringDiff {
    let mut difference = 0;
    let mut equal = String::new();

    for pair in str1.chars().zip(str2.chars()) {
        if pair.0 == pair.1 {
            equal.push_str(&pair.0.to_string())
        } else {
            difference += 1
        }
    }

    StringDiff { difference: difference, equal: equal }
}

/// Represents the difference between two Strings.
/// difference will the number of difference chars between the two
/// equal will be a string  with all the equal chars
struct StringDiff {
    difference: i32,
    equal: String
}

/// Get the contents of the input file as a String
fn get_input() -> String {
    match fs::read_to_string("input") {
        Ok(v) => return v,
        _ => String::new()
    }
}
