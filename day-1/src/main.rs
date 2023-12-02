use std::fs;

fn read_input_file(part: bool) -> u32 {
    fs::read_to_string("./part-one-input.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if part {
                line.to_string()
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
            } else {
                line.to_string()
            }
        })
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

fn main() {
    println!("Total sum for part one: {}", read_input_file(false));
    println!("Total sum for part two: {}", read_input_file(true));
}
