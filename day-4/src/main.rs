use std::fs;

fn part_one_and_two() -> (u32, u32) {
    let mut sum = 0;
    let mut total_games: Vec<u32> = vec![0; 211];

    let file = fs::read_to_string("./input.txt")
        .expect("File should be found at the path relative to src dir");

    let lines = file.lines().collect::<Vec<&str>>();

    let mut game_idx: usize = 0;
    for line in lines {
        total_games[game_idx] += 1;

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

            for game in game_idx + 1..num_winning_in_numbers + game_idx + 1 {
                total_games[game] += 1 * total_games[game_idx];
            }
        }

        game_idx += 1;
    }

    (sum / 2, total_games.iter().sum())
}

fn main() {
    let (part_one, part_two) = part_one_and_two();

    println!("Total for part one: {}", part_one);
    println!("Total for part two: {}", part_two);
}
