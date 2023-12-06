use std::fs;

fn part_one() -> u32 {
    let mut sum = 0;
    let file = fs::read_to_string("./input.txt")
        .expect("File should be found at the path relative to src dir");

    let lines = file.lines().collect::<Vec<&str>>();

    for line in lines {
        let idx_colon: usize = line.find(":").unwrap();
        let idx_bar: usize = line.find("|").unwrap();

        let winning_nums: Vec<u8> = line[idx_colon + 1..idx_bar]
            .trim()
            .split_whitespace()
            .map(|num: &str| num.parse::<u8>().unwrap())
            .collect();
        let numbers: Vec<u8> = line[idx_bar + 2..]
            .trim()
            .split_whitespace()
            .map(|num: &str| num.parse::<u8>().unwrap())
            .collect();

        let mut num_winning_in_numbers: usize = 0;
        for num in numbers {
            if winning_nums.contains(&num) {
                num_winning_in_numbers += 1;
            }
        }

        if num_winning_in_numbers > 0 {
            sum += 2 << (num_winning_in_numbers - 1);
        }
    }

    sum / 2
}

fn main() {
    println!("Total for part one: {}", part_one());
}
